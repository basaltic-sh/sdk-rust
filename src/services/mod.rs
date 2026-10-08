//! Generated typed service accessors.
use crate::Client;
pub mod audit;
impl Client {
    pub fn audit(&self) -> audit::AuditService {
        audit::AuditService {
            client: self.clone(),
        }
    }
}
pub mod billing;
impl Client {
    pub fn billing(&self) -> billing::BillingService {
        billing::BillingService {
            client: self.clone(),
        }
    }
}
pub mod catalog;
impl Client {
    pub fn catalog(&self) -> catalog::CatalogService {
        catalog::CatalogService {
            client: self.clone(),
        }
    }
}
pub mod certificate;
impl Client {
    pub fn certificate(&self) -> certificate::CertificateService {
        certificate::CertificateService {
            client: self.clone(),
        }
    }
}
pub mod compute;
impl Client {
    pub fn compute(&self) -> compute::ComputeService {
        compute::ComputeService {
            client: self.clone(),
        }
    }
}
pub mod dns;
impl Client {
    pub fn dns(&self) -> dns::DnsService {
        dns::DnsService {
            client: self.clone(),
        }
    }
}
pub mod iam;
impl Client {
    pub fn iam(&self) -> iam::IamService {
        iam::IamService {
            client: self.clone(),
        }
    }
}
pub mod kms;
impl Client {
    pub fn kms(&self) -> kms::KmsService {
        kms::KmsService {
            client: self.clone(),
        }
    }
}
pub mod loadbalancer;
impl Client {
    pub fn loadbalancer(&self) -> loadbalancer::LoadbalancerService {
        loadbalancer::LoadbalancerService {
            client: self.clone(),
        }
    }
}
pub mod network;
impl Client {
    pub fn network(&self) -> network::NetworkService {
        network::NetworkService {
            client: self.clone(),
        }
    }
}
pub mod quota;
impl Client {
    pub fn quota(&self) -> quota::QuotaService {
        quota::QuotaService {
            client: self.clone(),
        }
    }
}
pub mod secrets;
impl Client {
    pub fn secrets(&self) -> secrets::SecretsService {
        secrets::SecretsService {
            client: self.clone(),
        }
    }
}
pub mod storage;
impl Client {
    pub fn storage(&self) -> storage::StorageService {
        storage::StorageService {
            client: self.clone(),
        }
    }
}
pub mod telemetry;
impl Client {
    pub fn telemetry(&self) -> telemetry::TelemetryService {
        telemetry::TelemetryService {
            client: self.clone(),
        }
    }
}
pub mod workspace;
impl Client {
    pub fn workspace(&self) -> workspace::WorkspaceService {
        workspace::WorkspaceService {
            client: self.clone(),
        }
    }
}
