#!/usr/bin/env python3
"""Verify a source crate and compile an independent consumer of its extracted files."""
import hashlib
import json
import os
import subprocess
import tarfile
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    package = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]
    assert package["name"] == "basaltic-sdk-rust"
    assert package["repository"] == "https://github.com/basaltic-sh/sdk-rust"
    assert not package.get("authors"), "Do not embed developer identities"
    tag = os.environ.get("RELEASE_TAG")
    if tag: assert tag == "v" + package["version"], "Tag and crate version differ"
    subprocess.run(["cargo", "package", "--locked", "--allow-dirty"], cwd=ROOT, check=True)
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version=1"], cwd=ROOT))
    archive = Path(metadata["target_directory"]) / "package" / (package["name"] + "-" + package["version"] + ".crate")
    prefix = package["name"] + "-" + package["version"] + "/"
    with tempfile.TemporaryDirectory(prefix="basaltic-crate-check-") as tmp:
        destination = Path(tmp)
        exported = destination / prefix
        with tarfile.open(archive) as tar:
            seen = set()
            for member in tar.getmembers():
                assert member.isfile() and member.name.startswith(prefix), "Unexpected archive entry"
                relative = member.name[len(prefix):]
                assert relative and not Path(relative).is_absolute() and ".." not in Path(relative).parts
                assert relative not in seen, "Duplicate archive entry"
                seen.add(relative)
                assert relative.startswith("src/") or relative in {"Cargo.toml", "Cargo.toml.orig", "Cargo.lock", ".cargo_vcs_info.json", "README.md", "SECURITY.md", "LICENSE"}, relative
                content = tar.extractfile(member).read()
                path = exported / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(content)
                if relative.startswith("src/") or relative in {"README.md", "SECURITY.md", "LICENSE"}:
                    assert content == (ROOT / relative).read_bytes(), "Package differs: " + relative
                if relative == "Cargo.toml.orig": assert content == (ROOT / "Cargo.toml").read_bytes()
                if relative == ".cargo_vcs_info.json":
                    vcs = json.loads(content)
                    assert vcs["path_in_vcs"] == "", "Nested private path in archive"
                    # Hosted CI containers may use a different UID from checkout.
                    # Trust only this reviewed source directory for this command.
                    expected = subprocess.check_output(["git", "-c", "safe.directory=" + str(ROOT), "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
                    assert vcs["git"]["sha1"] == expected
            assert {"src/" + str(p.relative_to(ROOT / "src")) for p in (ROOT / "src").rglob("*.rs")} <= seen
            assert {"Cargo.toml", "Cargo.toml.orig", "Cargo.lock", "README.md", "SECURITY.md", "LICENSE"} <= seen
        consumer = destination / "consumer"
        (consumer / "src").mkdir(parents=True)
        (consumer / "Cargo.toml").write_text('[package]\nname="basaltic-consumer-check"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nbasaltic={package="basaltic-sdk-rust",path='+json.dumps(str(exported))+'}\n')
        (consumer / "src/main.rs").write_text('use basaltic::{Client, Config, Credentials, models::compute::{ListImagesQuery, GetImageResponse}};\nfn main() -> basaltic::Result<()> { let client = Client::new(Config { credentials: Credentials::bearer("fixture-token")?, region: "test-1".into(), ..Config::default() })?; let _: basaltic::Request<GetImageResponse> = client.compute().get_image("id"); let _stream = client.compute().list_images(&ListImagesQuery::default()).items(); let _binary = client.storage().get_object("bucket", "key"); Ok(()) }\n')
        env = {**os.environ, "CARGO_TARGET_DIR": str(Path(metadata["target_directory"]) / "consumer")}
        subprocess.run(["cargo", "run", "--quiet", "--offline", "--manifest-path", str(consumer / "Cargo.toml")], check=True, env=env)
    print(json.dumps({"package": package["name"], "version": package["version"], "files": len(seen), "source_bytes_match": True, "consumer_passed": True, "sha256": hashlib.sha256(archive.read_bytes()).hexdigest()}))


if __name__ == "__main__": main()
