# Rust API reference

All methods return owned request builders. Execute HTTP calls with `.send().await`, or WebSocket descriptors with `.prepare().await`. Pass per-request configuration using `.options(RequestOptions { ..Default::default() })`. Paged requests provide lazy `.pages()` and `.items()` streams.

## `audit.get_audit_log`

Get audit log entry

`GET /v1/audit-logs/{log_id}`

```rust
pub fn get_audit_log(&self, log_id: &str) -> Request<m::GetAuditLogResponse>
```

## `audit.list_audit_logs`

List audit logs

`GET /v1/audit-logs`

```rust
pub fn list_audit_logs(&self, query: &m::ListAuditLogsQuery) -> PagedRequest<m::ListAuditLogsResponse, m::ListAuditLogsItem>
```

## `billing.get_billing_profile`

Read the organization billing profile

`GET /v1/profile`

```rust
pub fn get_billing_profile(&self) -> Request<m::GetBillingProfileResponse>
```

## `billing.get_current_usage`

Get month-to-date usage total

`GET /v1/usage`

```rust
pub fn get_current_usage(&self) -> Request<m::GetCurrentUsageResponse>
```

## `billing.get_fiscal_invoice_xml`

Download issued NFS-e XML

`GET /v1/fiscal-invoices/{document_id}/xml`

```rust
pub fn get_fiscal_invoice_xml(&self, document_id: &str) -> BinaryRequest
```

## `billing.get_invoice`

Get an invoice with its line items

`GET /v1/invoices/{invoice_id}`

```rust
pub fn get_invoice(&self, invoice_id: &str) -> Request<m::GetInvoiceResponse>
```

## `billing.get_invoice_pdf`

Download an invoice as a PDF statement

`GET /v1/invoices/{invoice_id}/pdf`

```rust
pub fn get_invoice_pdf(&self, invoice_id: &str) -> BinaryRequest
```

## `billing.list_credits`

List credit grants

`GET /v1/credits`

```rust
pub fn list_credits(&self, query: &m::ListCreditsQuery) -> PagedRequest<m::ListCreditsResponse, m::ListCreditsItem>
```

## `billing.list_fiscal_invoices`

List issued fiscal invoices

`GET /v1/fiscal-invoices`

```rust
pub fn list_fiscal_invoices(&self, query: &m::ListFiscalInvoicesQuery) -> PagedRequest<m::ListFiscalInvoicesResponse, m::ListFiscalInvoicesItem>
```

## `billing.list_invoices`

List invoices

`GET /v1/invoices`

```rust
pub fn list_invoices(&self, query: &m::ListInvoicesQuery) -> PagedRequest<m::ListInvoicesResponse, m::ListInvoicesItem>
```

## `billing.list_payments`

List invoice payments

`GET /v1/payments`

```rust
pub fn list_payments(&self, query: &m::ListPaymentsQuery) -> PagedRequest<m::ListPaymentsResponse, m::ListPaymentsItem>
```

## `billing.list_prices`

List catalog prices

`GET /v1/prices`

```rust
pub fn list_prices(&self, query: &m::ListPricesQuery) -> PagedRequest<m::ListPricesResponse, m::ListPricesItem>
```

## `billing.list_transactions`

List ledger transactions

`GET /v1/transactions`

```rust
pub fn list_transactions(&self, query: &m::ListTransactionsQuery) -> PagedRequest<m::ListTransactionsResponse, m::ListTransactionsItem>
```

## `billing.update_billing_profile`

Save organization billing details

`PUT /v1/profile`

```rust
pub fn update_billing_profile(&self, body: &m::UpdateBillingProfileBody) -> Request<m::UpdateBillingProfileResponse>
```

## `catalog.get_region`

Get a region

`GET /v1/regions/{code}`

```rust
pub fn get_region(&self, code: &str) -> Request<m::GetRegionResponse>
```

## `catalog.list_regions`

List regions

`GET /v1/regions`

```rust
pub fn list_regions(&self, query: &m::ListRegionsQuery) -> PagedRequest<m::ListRegionsResponse, m::ListRegionsItem>
```

## `certificate.create_certificate`

Create certificate

`POST /v1/certificates`

```rust
pub fn create_certificate(&self, body: &m::CreateCertificateBody) -> Request<m::CreateCertificateResponse>
```

## `certificate.delete_certificate`

Delete certificate

`DELETE /v1/certificates/{certificate_id}`

```rust
pub fn delete_certificate(&self, certificate_id: &str) -> EmptyRequest
```

## `certificate.get_certificate`

Get certificate

`GET /v1/certificates/{certificate_id}`

```rust
pub fn get_certificate(&self, certificate_id: &str) -> Request<m::GetCertificateResponse>
```

## `certificate.get_certificate_material`

Fetch certificate material (leaf, chain, private key)

`GET /v1/certificates/{certificate_id}/material`

```rust
pub fn get_certificate_material(&self, certificate_id: &str) -> Request<m::GetCertificateMaterialResponse>
```

## `certificate.list_certificates`

List certificates

`GET /v1/certificates`

```rust
pub fn list_certificates(&self, query: &m::ListCertificatesQuery) -> PagedRequest<m::ListCertificatesResponse, m::ListCertificatesItem>
```

## `certificate.revoke_certificate`

Revoke certificate

`POST /v1/certificates/{certificate_id}/revoke`

```rust
pub fn revoke_certificate(&self, certificate_id: &str) -> Request<m::RevokeCertificateResponse>
```

## `compute.attach_instance_interface`

Attach an existing NIC to an instance

`POST /v1/instances/{instance_id}/interfaces`

```rust
pub fn attach_instance_interface(&self, instance_id: &str, body: &m::AttachInstanceInterfaceBody) -> Request<m::AttachInstanceInterfaceResponse>
```

## `compute.attach_instance_pool_floating_ip`

Give the pool a shared public address

`POST /v1/instance-pools/{pool_id}/floating-ips`

```rust
pub fn attach_instance_pool_floating_ip(&self, pool_id: &str, body: &m::AttachInstancePoolFloatingIpBody) -> Request<m::AttachInstancePoolFloatingIpResponse>
```

## `compute.attach_instance_volume`

Attach a data volume to an instance

`POST /v1/instances/{instance_id}/volumes`

```rust
pub fn attach_instance_volume(&self, instance_id: &str, body: &m::AttachInstanceVolumeBody) -> Request<m::AttachInstanceVolumeResponse>
```

## `compute.create_image`

Import an image from an object URL

`POST /v1/images`

```rust
pub fn create_image(&self, body: &m::CreateImageBody) -> Request<m::CreateImageResponse>
```

## `compute.create_instance`

Create instance

`POST /v1/instances`

```rust
pub fn create_instance(&self, body: &m::CreateInstanceBody) -> Request<m::CreateInstanceResponse>
```

## `compute.create_instance_pool`

Create an instance pool

`POST /v1/instance-pools`

```rust
pub fn create_instance_pool(&self, body: &m::CreateInstancePoolBody) -> Request<m::CreateInstancePoolResponse>
```

## `compute.create_serial_console_ticket`

Mint a ticket for the serial console

`POST /v1/instances/{instance_id}/console/ticket`

```rust
pub fn create_serial_console_ticket(&self, instance_id: &str) -> Request<m::CreateSerialConsoleTicketResponse>
```

## `compute.delete_image`

Delete an unused image

`DELETE /v1/images/{image_id}`

```rust
pub fn delete_image(&self, image_id: &str) -> Request<m::DeleteImageResponse>
```

## `compute.delete_instance`

Delete instance

`DELETE /v1/instances/{instance_id}`

```rust
pub fn delete_instance(&self, instance_id: &str) -> EmptyRequest
```

## `compute.delete_instance_pool`

Delete an instance pool

`DELETE /v1/instance-pools/{pool_id}`

```rust
pub fn delete_instance_pool(&self, pool_id: &str) -> EmptyRequest
```

## `compute.detach_instance_interface`

Detach an interface from a running instance

`DELETE /v1/instances/{instance_id}/interfaces/{interface_id}`

```rust
pub fn detach_instance_interface(&self, instance_id: &str, interface_id: &str) -> EmptyRequest
```

## `compute.detach_instance_pool_floating_ip`

Take a shared address off the pool

`DELETE /v1/instance-pools/{pool_id}/floating-ips/{floating_ip_id}`

```rust
pub fn detach_instance_pool_floating_ip(&self, pool_id: &str, floating_ip_id: &str) -> EmptyRequest
```

## `compute.detach_instance_volume`

Detach a data volume from an instance

`DELETE /v1/instances/{instance_id}/volumes/{volume_id}`

```rust
pub fn detach_instance_volume(&self, instance_id: &str, volume_id: &str) -> EmptyRequest
```

## `compute.get_console_output`

Get the instance's serial console output

`GET /v1/instances/{instance_id}/console/output`

```rust
pub fn get_console_output(&self, instance_id: &str, query: &m::GetConsoleOutputQuery) -> Request<m::GetConsoleOutputResponse>
```

## `compute.get_console_screenshot`

Capture the instance's display

`GET /v1/instances/{instance_id}/console/screenshot`

```rust
pub fn get_console_screenshot(&self, instance_id: &str) -> BinaryRequest
```

## `compute.get_flavor`

Get flavor

`GET /v1/flavors/{flavor_id}`

```rust
pub fn get_flavor(&self, flavor_id: &str) -> Request<m::GetFlavorResponse>
```

## `compute.get_image`

Get an image

`GET /v1/images/{image_id}`

```rust
pub fn get_image(&self, image_id: &str) -> Request<m::GetImageResponse>
```

## `compute.get_instance`

Get instance

`GET /v1/instances/{instance_id}`

```rust
pub fn get_instance(&self, instance_id: &str) -> Request<m::GetInstanceResponse>
```

## `compute.get_instance_pool`

Get an instance pool

`GET /v1/instance-pools/{pool_id}`

```rust
pub fn get_instance_pool(&self, pool_id: &str) -> Request<m::GetInstancePoolResponse>
```

## `compute.list_flavors`

List flavors

`GET /v1/flavors`

```rust
pub fn list_flavors(&self, query: &m::ListFlavorsQuery) -> PagedRequest<m::ListFlavorsResponse, m::ListFlavorsItem>
```

## `compute.list_image_catalog`

List the launch image catalog

`GET /v1/image-catalog`

```rust
pub fn list_image_catalog(&self, query: &m::ListImageCatalogQuery) -> PagedRequest<m::ListImageCatalogResponse, m::ListImageCatalogItem>
```

## `compute.list_images`

List images

`GET /v1/images`

```rust
pub fn list_images(&self, query: &m::ListImagesQuery) -> PagedRequest<m::ListImagesResponse, m::ListImagesItem>
```

## `compute.list_instance_interfaces`

List the instance's network interfaces

`GET /v1/instances/{instance_id}/interfaces`

```rust
pub fn list_instance_interfaces(&self, instance_id: &str, query: &m::ListInstanceInterfacesQuery) -> PagedRequest<m::ListInstanceInterfacesResponse, m::ListInstanceInterfacesItem>
```

## `compute.list_instance_pool_floating_ips`

List the pool's shared public addresses

`GET /v1/instance-pools/{pool_id}/floating-ips`

```rust
pub fn list_instance_pool_floating_ips(&self, pool_id: &str, query: &m::ListInstancePoolFloatingIpsQuery) -> PagedRequest<m::ListInstancePoolFloatingIpsResponse, m::ListInstancePoolFloatingIpsItem>
```

## `compute.list_instance_pools`

List instance pools

`GET /v1/instance-pools`

```rust
pub fn list_instance_pools(&self, query: &m::ListInstancePoolsQuery) -> PagedRequest<m::ListInstancePoolsResponse, m::ListInstancePoolsItem>
```

## `compute.list_instance_volumes`

List the instance's attached volumes

`GET /v1/instances/{instance_id}/volumes`

```rust
pub fn list_instance_volumes(&self, instance_id: &str, query: &m::ListInstanceVolumesQuery) -> PagedRequest<m::ListInstanceVolumesResponse, m::ListInstanceVolumesItem>
```

## `compute.list_instances`

List instances

`GET /v1/instances`

```rust
pub fn list_instances(&self, query: &m::ListInstancesQuery) -> PagedRequest<m::ListInstancesResponse, m::ListInstancesItem>
```

## `compute.list_pool_instances`

List a pool's instances

`GET /v1/instance-pools/{pool_id}/instances`

```rust
pub fn list_pool_instances(&self, pool_id: &str, query: &m::ListPoolInstancesQuery) -> PagedRequest<m::ListPoolInstancesResponse, m::ListPoolInstancesItem>
```

## `compute.reboot_instance`

Reboot instance

`POST /v1/instances/{instance_id}/reboot`

```rust
pub fn reboot_instance(&self, instance_id: &str, body: Option<&m::RebootInstanceBody>) -> EmptyRequest
```

## `compute.refresh_instance_pool`

Roll every member onto the pool's current launch template

`POST /v1/instance-pools/{pool_id}/refresh`

```rust
pub fn refresh_instance_pool(&self, pool_id: &str) -> Request<m::RefreshInstancePoolResponse>
```

## `compute.reinstall_instance`

Reinstall instance

`POST /v1/instances/{instance_id}/reinstall`

```rust
pub fn reinstall_instance(&self, instance_id: &str, body: Option<&m::ReinstallInstanceBody>) -> EmptyRequest
```

## `compute.resize_instance`

Resize instance

`POST /v1/instances/{instance_id}/resize`

```rust
pub fn resize_instance(&self, instance_id: &str, body: &m::ResizeInstanceBody) -> EmptyRequest
```

## `compute.start_instance`

Start instance

`POST /v1/instances/{instance_id}/start`

```rust
pub fn start_instance(&self, instance_id: &str) -> EmptyRequest
```

## `compute.start_serial_console`

Open an interactive serial console

`GET /v1/instances/{instance_id}/console/serial`

```rust
pub fn start_serial_console(&self, instance_id: &str, query: &m::StartSerialConsoleQuery) -> WebSocketRequest
```

## `compute.stop_instance`

Stop instance

`POST /v1/instances/{instance_id}/stop`

```rust
pub fn stop_instance(&self, instance_id: &str) -> EmptyRequest
```

## `compute.update_image`

Update an image's metadata

`PATCH /v1/images/{image_id}`

```rust
pub fn update_image(&self, image_id: &str, body: &m::UpdateImageBody) -> Request<m::UpdateImageResponse>
```

## `compute.update_instance`

Update instance

`PATCH /v1/instances/{instance_id}`

```rust
pub fn update_instance(&self, instance_id: &str, body: &m::UpdateInstanceBody) -> Request<m::UpdateInstanceResponse>
```

## `compute.update_instance_pool`

Update an instance pool's description, size, tags or launch template

`PATCH /v1/instance-pools/{pool_id}`

```rust
pub fn update_instance_pool(&self, pool_id: &str, body: &m::UpdateInstancePoolBody) -> Request<m::UpdateInstancePoolResponse>
```

## `compute.update_instance_pool_member_protection`

Update member scale-in protection

`PATCH /v1/instance-pools/{pool_id}/instances/{instance_id}`

```rust
pub fn update_instance_pool_member_protection(&self, pool_id: &str, instance_id: &str, body: &m::UpdateInstancePoolMemberProtectionBody) -> Request<m::UpdateInstancePoolMemberProtectionResponse>
```

## `compute.update_instance_volume_attachment`

Update a volume attachment's settings

`PATCH /v1/instances/{instance_id}/volumes/{volume_id}`

```rust
pub fn update_instance_volume_attachment(&self, instance_id: &str, volume_id: &str, body: &m::UpdateInstanceVolumeAttachmentBody) -> EmptyRequest
```

## `dns.associate_zone_vpc`

Associate a VPC with a private zone

`POST /v1/zones/{zone_id}/vpc-associations`

```rust
pub fn associate_zone_vpc(&self, zone_id: &str, body: &m::AssociateZoneVPCBody) -> Request<m::AssociateZoneVPCResponse>
```

## `dns.create_record`

Create record

`POST /v1/zones/{zone_id}/records`

```rust
pub fn create_record(&self, zone_id: &str, body: &m::CreateRecordBody) -> Request<m::CreateRecordResponse>
```

## `dns.create_zone`

Create zone

`POST /v1/zones`

```rust
pub fn create_zone(&self, body: &m::CreateZoneBody) -> Request<m::CreateZoneResponse>
```

## `dns.delete_record`

Delete record

`DELETE /v1/zones/{zone_id}/records/{record_id}`

```rust
pub fn delete_record(&self, zone_id: &str, record_id: &str) -> EmptyRequest
```

## `dns.delete_zone`

Delete zone

`DELETE /v1/zones/{zone_id}`

```rust
pub fn delete_zone(&self, zone_id: &str) -> EmptyRequest
```

## `dns.delete_zone_record_import`

Discard the record-import outcome

`DELETE /v1/zones/{zone_id}/record-import`

```rust
pub fn delete_zone_record_import(&self, zone_id: &str) -> EmptyRequest
```

## `dns.dissociate_zone_vpc`

Dissociate a VPC from a private zone

`DELETE /v1/zones/{zone_id}/vpc-associations/{vpc_id}`

```rust
pub fn dissociate_zone_vpc(&self, zone_id: &str, vpc_id: &str) -> EmptyRequest
```

## `dns.export_zone_file`

Export the zone as a zone file

`GET /v1/zones/{zone_id}/export`

```rust
pub fn export_zone_file(&self, zone_id: &str) -> BinaryRequest
```

## `dns.get_record`

Get record

`GET /v1/zones/{zone_id}/records/{record_id}`

```rust
pub fn get_record(&self, zone_id: &str, record_id: &str) -> Request<m::GetRecordResponse>
```

## `dns.get_zone`

Get zone

`GET /v1/zones/{zone_id}`

```rust
pub fn get_zone(&self, zone_id: &str) -> Request<m::GetZoneResponse>
```

## `dns.get_zone_record_import`

Get the record-import outcome

`GET /v1/zones/{zone_id}/record-import`

```rust
pub fn get_zone_record_import(&self, zone_id: &str) -> Request<m::GetZoneRecordImportResponse>
```

## `dns.import_zone_file`

Import a zone file

`POST /v1/zones/{zone_id}/import`

```rust
pub fn import_zone_file(&self, zone_id: &str, body: &m::ImportZoneFileBody) -> Request<m::ImportZoneFileResponse>
```

## `dns.list_records`

List records

`GET /v1/zones/{zone_id}/records`

```rust
pub fn list_records(&self, zone_id: &str, query: &m::ListRecordsQuery) -> PagedRequest<m::ListRecordsResponse, m::ListRecordsItem>
```

## `dns.list_zone_vpc_associations`

List VPC associations

`GET /v1/zones/{zone_id}/vpc-associations`

```rust
pub fn list_zone_vpc_associations(&self, zone_id: &str, query: &m::ListZoneVPCAssociationsQuery) -> PagedRequest<m::ListZoneVPCAssociationsResponse, m::ListZoneVPCAssociationsItem>
```

## `dns.list_zones`

List zones

`GET /v1/zones`

```rust
pub fn list_zones(&self, query: &m::ListZonesQuery) -> PagedRequest<m::ListZonesResponse, m::ListZonesItem>
```

## `dns.update_record`

Update record

`PATCH /v1/zones/{zone_id}/records/{record_id}`

```rust
pub fn update_record(&self, zone_id: &str, record_id: &str, body: &m::UpdateRecordBody) -> Request<m::UpdateRecordResponse>
```

## `dns.update_zone`

Update zone

`PATCH /v1/zones/{zone_id}`

```rust
pub fn update_zone(&self, zone_id: &str, body: &m::UpdateZoneBody) -> Request<m::UpdateZoneResponse>
```

## `dns.verify_zone_ownership`

Verify zone ownership

`POST /v1/zones/{zone_id}/verify-ownership`

```rust
pub fn verify_zone_ownership(&self, zone_id: &str) -> Request<m::VerifyZoneOwnershipResponse>
```

## `email.add_suppression_override`

Add a suppression override address

`POST /v1/configuration-sets/{configuration_set_id}/suppression-overrides`

```rust
pub fn add_suppression_override(&self, configuration_set_id: &str, body: &m::AddSuppressionOverrideBody) -> Request<m::AddSuppressionOverrideResponse>
```

## `email.begin_dkim_rotation`

Begin DKIM key rotation

`POST /v1/identities/{identity_id}/dkim/rotate`

```rust
pub fn begin_dkim_rotation(&self, identity_id: &str) -> Request<m::BeginDKIMRotationResponse>
```

## `email.cancel_dkim_rotation`

Cancel in-progress DKIM rotation

`DELETE /v1/identities/{identity_id}/dkim/rotate`

```rust
pub fn cancel_dkim_rotation(&self, identity_id: &str) -> Request<m::CancelDKIMRotationResponse>
```

## `email.create_configuration_set`

Create a configuration set

`POST /v1/configuration-sets`

```rust
pub fn create_configuration_set(&self, body: &m::CreateConfigurationSetBody) -> Request<m::CreateConfigurationSetResponse>
```

## `email.create_ip_pool`

Create an IP pool

`POST /v1/ip-pools`

```rust
pub fn create_ip_pool(&self, body: &m::CreateIPPoolBody) -> Request<m::CreateIPPoolResponse>
```

## `email.create_identity`

Create a sender identity

`POST /v1/identities`

```rust
pub fn create_identity(&self, body: &m::CreateIdentityBody) -> Request<m::CreateIdentityResponse>
```

## `email.create_recipient_rule`

Create recipient rule

`POST /v1/identities/{identity_id}/recipient-rules`

```rust
pub fn create_recipient_rule(&self, identity_id: &str, body: &m::CreateRecipientRuleBody) -> Request<m::CreateRecipientRuleResponse>
```

## `email.create_suppression`

Add a suppression entry

`POST /v1/suppressions`

```rust
pub fn create_suppression(&self, body: &m::CreateSuppressionBody) -> Request<m::CreateSuppressionResponse>
```

## `email.create_template`

Create a template

`POST /v1/templates`

```rust
pub fn create_template(&self, body: &m::CreateTemplateBody) -> Request<m::CreateTemplateResponse>
```

## `email.delete_configuration_set`

Delete a configuration set

`DELETE /v1/configuration-sets/{configuration_set_id}`

```rust
pub fn delete_configuration_set(&self, configuration_set_id: &str) -> EmptyRequest
```

## `email.delete_ip_pool`

Delete an IP pool

`DELETE /v1/ip-pools/{ip_pool_id}`

```rust
pub fn delete_ip_pool(&self, ip_pool_id: &str) -> EmptyRequest
```

## `email.delete_identity`

Delete an identity

`DELETE /v1/identities/{identity_id}`

```rust
pub fn delete_identity(&self, identity_id: &str) -> EmptyRequest
```

## `email.delete_recipient_rule`

Delete recipient rule

`DELETE /v1/identities/{identity_id}/recipient-rules/{rule_id}`

```rust
pub fn delete_recipient_rule(&self, identity_id: &str, rule_id: &str) -> EmptyRequest
```

## `email.delete_suppression`

Delete suppression

`DELETE /v1/suppressions/{suppression_id}`

```rust
pub fn delete_suppression(&self, suppression_id: &str) -> EmptyRequest
```

## `email.delete_template`

Delete a template

`DELETE /v1/templates/{template_id}`

```rust
pub fn delete_template(&self, template_id: &str) -> EmptyRequest
```

## `email.describe_configuration_set`

Describe a configuration set

`GET /v1/configuration-sets/{configuration_set_id}`

```rust
pub fn describe_configuration_set(&self, configuration_set_id: &str) -> Request<m::DescribeConfigurationSetResponse>
```

## `email.describe_dedicated_ip`

Describe a dedicated IP

`GET /v1/dedicated-ips/{dedicated_ip_id}`

```rust
pub fn describe_dedicated_ip(&self, dedicated_ip_id: &str) -> Request<m::DescribeDedicatedIPResponse>
```

## `email.describe_ip_pool`

Describe an IP pool

`GET /v1/ip-pools/{ip_pool_id}`

```rust
pub fn describe_ip_pool(&self, ip_pool_id: &str) -> Request<m::DescribeIPPoolResponse>
```

## `email.describe_identity`

Describe an identity

`GET /v1/identities/{identity_id}`

```rust
pub fn describe_identity(&self, identity_id: &str) -> Request<m::DescribeIdentityResponse>
```

## `email.describe_received_message`

Get received message metadata

`GET /v1/received-messages/{message_id}`

```rust
pub fn describe_received_message(&self, message_id: &str) -> Request<m::DescribeReceivedMessageResponse>
```

## `email.describe_recipient_rule`

Get recipient rule

`GET /v1/identities/{identity_id}/recipient-rules/{rule_id}`

```rust
pub fn describe_recipient_rule(&self, identity_id: &str, rule_id: &str) -> Request<m::DescribeRecipientRuleResponse>
```

## `email.describe_sent_message`

Describe a sent message

`GET /v1/sent-messages/{message_id}`

```rust
pub fn describe_sent_message(&self, message_id: &str) -> Request<m::DescribeSentMessageResponse>
```

## `email.describe_suppression`

Get suppression

`GET /v1/suppressions/{suppression_id}`

```rust
pub fn describe_suppression(&self, suppression_id: &str) -> Request<m::DescribeSuppressionResponse>
```

## `email.describe_template`

Describe a template

`GET /v1/templates/{template_id}`

```rust
pub fn describe_template(&self, template_id: &str) -> Request<m::DescribeTemplateResponse>
```

## `email.download_received_attachment`

Download received attachment

`GET /v1/received-messages/{message_id}/attachments/{attachment_id}`

```rust
pub fn download_received_attachment(&self, message_id: &str, attachment_id: &str) -> BinaryRequest
```

## `email.download_received_message`

Download received message

`GET /v1/received-messages/{message_id}/raw`

```rust
pub fn download_received_message(&self, message_id: &str) -> BinaryRequest
```

## `email.download_sent_attachment`

Download sent attachment

`GET /v1/sent-messages/{message_id}/attachments/{attachment_id}`

```rust
pub fn download_sent_attachment(&self, message_id: &str, attachment_id: &str) -> BinaryRequest
```

## `email.download_sent_message`

Download sent message

`GET /v1/sent-messages/{message_id}/raw`

```rust
pub fn download_sent_message(&self, message_id: &str) -> BinaryRequest
```

## `email.finalize_dkim_rotation`

Finalize DKIM key rotation

`POST /v1/identities/{identity_id}/dkim/rotate/finalize`

```rust
pub fn finalize_dkim_rotation(&self, identity_id: &str) -> Request<m::FinalizeDKIMRotationResponse>
```

## `email.get_received_message_content`

Get received message content

`GET /v1/received-messages/{message_id}/content`

```rust
pub fn get_received_message_content(&self, message_id: &str) -> Request<m::GetReceivedMessageContentResponse>
```

## `email.get_receiving_configuration`

Get receiving configuration

`GET /v1/identities/{identity_id}/receiving`

```rust
pub fn get_receiving_configuration(&self, identity_id: &str) -> Request<m::GetReceivingConfigurationResponse>
```

## `email.get_sent_message_content`

Get sent message content

`GET /v1/sent-messages/{message_id}/content`

```rust
pub fn get_sent_message_content(&self, message_id: &str) -> Request<m::GetSentMessageContentResponse>
```

## `email.get_sent_message_engagement`

Sent message engagement

`GET /v1/sent-messages/{message_id}/engagement`

```rust
pub fn get_sent_message_engagement(&self, message_id: &str) -> Request<m::GetSentMessageEngagementResponse>
```

## `email.list_configuration_sets`

List configuration sets

`GET /v1/configuration-sets`

```rust
pub fn list_configuration_sets(&self, query: &m::ListConfigurationSetsQuery) -> PagedRequest<m::ListConfigurationSetsResponse, m::ListConfigurationSetsItem>
```

## `email.list_dmarc_reports`

List DMARC report records

`GET /v1/identities/{identity_id}/dmarc-reports`

```rust
pub fn list_dmarc_reports(&self, identity_id: &str, query: &m::ListDMARCReportsQuery) -> PagedRequest<m::ListDMARCReportsResponse, m::ListDMARCReportsItem>
```

## `email.list_dedicated_i_ps`

List dedicated IPs

`GET /v1/dedicated-ips`

```rust
pub fn list_dedicated_i_ps(&self, query: &m::ListDedicatedIPsQuery) -> PagedRequest<m::ListDedicatedIPsResponse, m::ListDedicatedIPsItem>
```

## `email.list_ip_pools`

List IP pools

`GET /v1/ip-pools`

```rust
pub fn list_ip_pools(&self, query: &m::ListIPPoolsQuery) -> PagedRequest<m::ListIPPoolsResponse, m::ListIPPoolsItem>
```

## `email.list_identities`

List sender identities

`GET /v1/identities`

```rust
pub fn list_identities(&self, query: &m::ListIdentitiesQuery) -> PagedRequest<m::ListIdentitiesResponse, m::ListIdentitiesItem>
```

## `email.list_received_messages`

List received messages

`GET /v1/received-messages`

```rust
pub fn list_received_messages(&self, query: &m::ListReceivedMessagesQuery) -> PagedRequest<m::ListReceivedMessagesResponse, m::ListReceivedMessagesItem>
```

## `email.list_recipient_rules`

List recipient rules

`GET /v1/identities/{identity_id}/recipient-rules`

```rust
pub fn list_recipient_rules(&self, identity_id: &str, query: &m::ListRecipientRulesQuery) -> PagedRequest<m::ListRecipientRulesResponse, m::ListRecipientRulesItem>
```

## `email.list_rejections`

List API-side send rejections

`GET /v1/rejections`

```rust
pub fn list_rejections(&self, query: &m::ListRejectionsQuery) -> PagedRequest<m::ListRejectionsResponse, m::ListRejectionsItem>
```

## `email.list_sent_messages`

List sent messages

`GET /v1/sent-messages`

```rust
pub fn list_sent_messages(&self, query: &m::ListSentMessagesQuery) -> PagedRequest<m::ListSentMessagesResponse, m::ListSentMessagesItem>
```

## `email.list_suppressions`

List suppression entries

`GET /v1/suppressions`

```rust
pub fn list_suppressions(&self, query: &m::ListSuppressionsQuery) -> PagedRequest<m::ListSuppressionsResponse, m::ListSuppressionsItem>
```

## `email.list_templates`

List templates

`GET /v1/templates`

```rust
pub fn list_templates(&self, query: &m::ListTemplatesQuery) -> PagedRequest<m::ListTemplatesResponse, m::ListTemplatesItem>
```

## `email.release_dedicated_ip`

Release a dedicated IP

`DELETE /v1/dedicated-ips/{dedicated_ip_id}`

```rust
pub fn release_dedicated_ip(&self, dedicated_ip_id: &str) -> EmptyRequest
```

## `email.remove_suppression_override`

Remove a suppression override address

`DELETE /v1/configuration-sets/{configuration_set_id}/suppression-overrides/{address}`

```rust
pub fn remove_suppression_override(&self, configuration_set_id: &str, address: &str) -> Request<m::RemoveSuppressionOverrideResponse>
```

## `email.request_dedicated_ip`

Request a dedicated IP

`POST /v1/dedicated-ips`

```rust
pub fn request_dedicated_ip(&self, body: Option<&m::RequestDedicatedIPBody>) -> Request<m::RequestDedicatedIPResponse>
```

## `email.send_message`

Send a message

`POST /v1/sent-messages`

```rust
pub fn send_message(&self, body: &m::SendMessageBody) -> Request<m::SendMessageResponse>
```

## `email.send_message_batch`

Send message batch

`POST /v1/sent-messages/batch`

```rust
pub fn send_message_batch(&self, body: &m::SendMessageBatchBody) -> Request<m::SendMessageBatchResponse>
```

## `email.set_bimi`

Set BIMI logo / VMC URLs

`PATCH /v1/identities/{identity_id}/bimi`

```rust
pub fn set_bimi(&self, identity_id: &str, body: &m::SetBIMIBody) -> Request<m::SetBIMIResponse>
```

## `email.set_mail_from_domain`

Set custom MAIL FROM domain

`PATCH /v1/identities/{identity_id}/mail-from-domain`

```rust
pub fn set_mail_from_domain(&self, identity_id: &str, body: &m::SetMailFromDomainBody) -> Request<m::SetMailFromDomainResponse>
```

## `email.update_configuration_set`

Update a configuration set

`PATCH /v1/configuration-sets/{configuration_set_id}`

```rust
pub fn update_configuration_set(&self, configuration_set_id: &str, body: &m::UpdateConfigurationSetBody) -> Request<m::UpdateConfigurationSetResponse>
```

## `email.update_dedicated_ip`

Update a dedicated IP

`PATCH /v1/dedicated-ips/{dedicated_ip_id}`

```rust
pub fn update_dedicated_ip(&self, dedicated_ip_id: &str, body: &m::UpdateDedicatedIPBody) -> Request<m::UpdateDedicatedIPResponse>
```

## `email.update_ip_pool`

Update an IP pool

`PATCH /v1/ip-pools/{ip_pool_id}`

```rust
pub fn update_ip_pool(&self, ip_pool_id: &str, body: &m::UpdateIPPoolBody) -> Request<m::UpdateIPPoolResponse>
```

## `email.update_identity`

Update identity

`PATCH /v1/identities/{identity_id}`

```rust
pub fn update_identity(&self, identity_id: &str, body: &m::UpdateIdentityBody) -> Request<m::UpdateIdentityResponse>
```

## `email.update_receiving_configuration`

Update receiving configuration

`PATCH /v1/identities/{identity_id}/receiving`

```rust
pub fn update_receiving_configuration(&self, identity_id: &str, body: &m::UpdateReceivingConfigurationBody) -> EmptyRequest
```

## `email.update_recipient_rule`

Update recipient rule

`PATCH /v1/identities/{identity_id}/recipient-rules/{rule_id}`

```rust
pub fn update_recipient_rule(&self, identity_id: &str, rule_id: &str, body: &m::UpdateRecipientRuleBody) -> Request<m::UpdateRecipientRuleResponse>
```

## `email.update_suppression`

Update suppression

`PATCH /v1/suppressions/{suppression_id}`

```rust
pub fn update_suppression(&self, suppression_id: &str, body: &m::UpdateSuppressionBody) -> Request<m::UpdateSuppressionResponse>
```

## `email.update_template`

Update a template

`PATCH /v1/templates/{template_id}`

```rust
pub fn update_template(&self, template_id: &str, body: &m::UpdateTemplateBody) -> Request<m::UpdateTemplateResponse>
```

## `email.verify_bimi`

Verify BIMI logo (+ optional VMC)

`POST /v1/identities/{identity_id}/bimi/verify`

```rust
pub fn verify_bimi(&self, identity_id: &str) -> Request<m::VerifyBIMIResponse>
```

## `email.verify_identity`

Trigger DNS verification

`POST /v1/identities/{identity_id}/verify`

```rust
pub fn verify_identity(&self, identity_id: &str) -> Request<m::VerifyIdentityResponse>
```

## `email.verify_mail_from_domain`

Verify custom MAIL FROM domain MX

`POST /v1/identities/{identity_id}/mail-from-domain/verify`

```rust
pub fn verify_mail_from_domain(&self, identity_id: &str) -> Request<m::VerifyMailFromDomainResponse>
```

## `iam.assume_role`

Assume role

`POST /v1/assume-role`

```rust
pub fn assume_role(&self, body: &m::AssumeRoleBody) -> Request<m::AssumeRoleResponse>
```

## `iam.assume_role_with_web_identity`

Assume role with web identity

`POST /v1/assume-role-with-web-identity`

```rust
pub fn assume_role_with_web_identity(&self, body: &m::AssumeRoleWithWebIdentityBody) -> Request<m::AssumeRoleWithWebIdentityResponse>
```

## `iam.attach_role_policy`

Attach policy to role

`POST /v1/roles/{role_id}/policies`

```rust
pub fn attach_role_policy(&self, role_id: &str, body: &m::AttachRolePolicyBody) -> EmptyRequest
```

## `iam.attach_service_account_policy`

Attach policy to service account

`POST /v1/service-accounts/{service_account_id}/policies`

```rust
pub fn attach_service_account_policy(&self, service_account_id: &str, body: &m::AttachServiceAccountPolicyBody) -> EmptyRequest
```

## `iam.create_oidc_provider`

Create an OIDC provider

`POST /v1/oidc-providers`

```rust
pub fn create_oidc_provider(&self, body: &m::CreateOIDCProviderBody) -> Request<m::CreateOIDCProviderResponse>
```

## `iam.create_personal_ssh_key`

Add personal SSH key

`POST /v1/auth/ssh-keys`

```rust
pub fn create_personal_ssh_key(&self, body: &m::CreatePersonalSSHKeyBody) -> Request<m::CreatePersonalSSHKeyResponse>
```

## `iam.create_policy`

Create policy

`POST /v1/policies`

```rust
pub fn create_policy(&self, body: &m::CreatePolicyBody) -> Request<m::CreatePolicyResponse>
```

## `iam.create_role`

Create role

`POST /v1/roles`

```rust
pub fn create_role(&self, body: &m::CreateRoleBody) -> Request<m::CreateRoleResponse>
```

## `iam.create_service_account`

Create service account

`POST /v1/service-accounts`

```rust
pub fn create_service_account(&self, body: &m::CreateServiceAccountBody) -> Request<m::CreateServiceAccountResponse>
```

## `iam.create_service_account_credential`

Create credential

`POST /v1/service-accounts/{service_account_id}/credentials`

```rust
pub fn create_service_account_credential(&self, service_account_id: &str, body: &m::CreateServiceAccountCredentialBody) -> Request<m::CreateServiceAccountCredentialResponse>
```

## `iam.create_service_account_ssh_key`

Add service-account SSH key

`POST /v1/service-accounts/{service_account_id}/ssh-keys`

```rust
pub fn create_service_account_ssh_key(&self, service_account_id: &str, body: &m::CreateServiceAccountSSHKeyBody) -> Request<m::CreateServiceAccountSSHKeyResponse>
```

## `iam.delete_oidc_provider`

Delete an OIDC provider

`DELETE /v1/oidc-providers/{provider_id}`

```rust
pub fn delete_oidc_provider(&self, provider_id: &str) -> EmptyRequest
```

## `iam.delete_personal_ssh_key`

Revoke personal SSH key

`DELETE /v1/auth/ssh-keys/{ssh_key_id}`

```rust
pub fn delete_personal_ssh_key(&self, ssh_key_id: &str) -> EmptyRequest
```

## `iam.delete_policy`

Delete policy

`DELETE /v1/policies/{policy_id}`

```rust
pub fn delete_policy(&self, policy_id: &str) -> EmptyRequest
```

## `iam.delete_role`

Delete role

`DELETE /v1/roles/{role_id}`

```rust
pub fn delete_role(&self, role_id: &str) -> EmptyRequest
```

## `iam.delete_role_inline_policy`

Delete a role's inline policy by name

`DELETE /v1/roles/{role_id}/inline-policies/{policy_name}`

```rust
pub fn delete_role_inline_policy(&self, role_id: &str, policy_name: &str) -> EmptyRequest
```

## `iam.delete_service_account`

Delete service account

`DELETE /v1/service-accounts/{service_account_id}`

```rust
pub fn delete_service_account(&self, service_account_id: &str) -> EmptyRequest
```

## `iam.delete_service_account_credential`

Delete credential

`DELETE /v1/service-accounts/{service_account_id}/credentials/{credential_id}`

```rust
pub fn delete_service_account_credential(&self, service_account_id: &str, credential_id: &str) -> EmptyRequest
```

## `iam.delete_service_account_inline_policy`

Delete a service account's inline policy by name

`DELETE /v1/service-accounts/{service_account_id}/inline-policies/{policy_name}`

```rust
pub fn delete_service_account_inline_policy(&self, service_account_id: &str, policy_name: &str) -> EmptyRequest
```

## `iam.delete_service_account_ssh_key`

Revoke service-account SSH key

`DELETE /v1/service-accounts/{service_account_id}/ssh-keys/{ssh_key_id}`

```rust
pub fn delete_service_account_ssh_key(&self, service_account_id: &str, ssh_key_id: &str) -> EmptyRequest
```

## `iam.detach_role_policy`

Detach policy from role

`DELETE /v1/roles/{role_id}/policies/{policy_id}`

```rust
pub fn detach_role_policy(&self, role_id: &str, policy_id: &str) -> EmptyRequest
```

## `iam.detach_service_account_policy`

Detach policy from service account

`DELETE /v1/service-accounts/{service_account_id}/policies/{policy_id}`

```rust
pub fn detach_service_account_policy(&self, service_account_id: &str, policy_id: &str) -> EmptyRequest
```

## `iam.get_oauth_token`

Exchange an access key for a bearer token

`POST /v1/oauth/token`

```rust
pub fn get_oauth_token(&self, body: &m::GetOAuthTokenBody) -> Request<m::GetOAuthTokenResponse>
```

## `iam.get_oidc_provider`

Get an OIDC provider

`GET /v1/oidc-providers/{provider_id}`

```rust
pub fn get_oidc_provider(&self, provider_id: &str) -> Request<m::GetOIDCProviderResponse>
```

## `iam.get_personal_linux_identity`

Get personal Linux identity

`GET /v1/auth/linux-identity`

```rust
pub fn get_personal_linux_identity(&self) -> Request<m::GetPersonalLinuxIdentityResponse>
```

## `iam.get_policy`

Get policy

`GET /v1/policies/{policy_id}`

```rust
pub fn get_policy(&self, policy_id: &str) -> Request<m::GetPolicyResponse>
```

## `iam.get_role`

Get role

`GET /v1/roles/{role_id}`

```rust
pub fn get_role(&self, role_id: &str) -> Request<m::GetRoleResponse>
```

## `iam.get_role_inline_policy`

Get a role's inline policy by name

`GET /v1/roles/{role_id}/inline-policies/{policy_name}`

```rust
pub fn get_role_inline_policy(&self, role_id: &str, policy_name: &str) -> Request<m::GetRoleInlinePolicyResponse>
```

## `iam.get_role_permission_boundary`

Get a role's permission boundary

`GET /v1/roles/{role_id}/permission-boundary`

```rust
pub fn get_role_permission_boundary(&self, role_id: &str) -> Request<m::GetRolePermissionBoundaryResponse>
```

## `iam.get_sts_session`

Get STS session

`GET /v1/sts-sessions/{session_id}`

```rust
pub fn get_sts_session(&self, session_id: &str) -> Request<m::GetSTSSessionResponse>
```

## `iam.get_service_account`

Get service account

`GET /v1/service-accounts/{service_account_id}`

```rust
pub fn get_service_account(&self, service_account_id: &str) -> Request<m::GetServiceAccountResponse>
```

## `iam.get_service_account_inline_policy`

Get a service account's inline policy by name

`GET /v1/service-accounts/{service_account_id}/inline-policies/{policy_name}`

```rust
pub fn get_service_account_inline_policy(&self, service_account_id: &str, policy_name: &str) -> Request<m::GetServiceAccountInlinePolicyResponse>
```

## `iam.get_service_account_linux_identity`

Get serviceaccount Linux identity

`GET /v1/service-accounts/{service_account_id}/linux-identity`

```rust
pub fn get_service_account_linux_identity(&self, service_account_id: &str) -> Request<m::GetServiceAccountLinuxIdentityResponse>
```

## `iam.get_service_account_permission_boundary`

Get a service account's permission boundary

`GET /v1/service-accounts/{service_account_id}/permission-boundary`

```rust
pub fn get_service_account_permission_boundary(&self, service_account_id: &str) -> Request<m::GetServiceAccountPermissionBoundaryResponse>
```

## `iam.list_oidc_providers`

List OIDC providers

`GET /v1/oidc-providers`

```rust
pub fn list_oidc_providers(&self, query: &m::ListOIDCProvidersQuery) -> PagedRequest<m::ListOIDCProvidersResponse, m::ListOIDCProvidersItem>
```

## `iam.list_personal_ssh_keys`

List personal SSH keys

`GET /v1/auth/ssh-keys`

```rust
pub fn list_personal_ssh_keys(&self) -> PagedRequest<m::ListPersonalSSHKeysResponse, m::ListPersonalSSHKeysItem>
```

## `iam.list_policies`

List policies

`GET /v1/policies`

```rust
pub fn list_policies(&self, query: &m::ListPoliciesQuery) -> PagedRequest<m::ListPoliciesResponse, m::ListPoliciesItem>
```

## `iam.list_policy_roles`

List roles with policy

`GET /v1/policies/{policy_id}/roles`

```rust
pub fn list_policy_roles(&self, policy_id: &str, query: &m::ListPolicyRolesQuery) -> PagedRequest<m::ListPolicyRolesResponse, m::ListPolicyRolesItem>
```

## `iam.list_policy_service_accounts`

List service accounts with policy

`GET /v1/policies/{policy_id}/service-accounts`

```rust
pub fn list_policy_service_accounts(&self, policy_id: &str, query: &m::ListPolicyServiceAccountsQuery) -> PagedRequest<m::ListPolicyServiceAccountsResponse, m::ListPolicyServiceAccountsItem>
```

## `iam.list_role_inline_policies`

List a role's inline policies

`GET /v1/roles/{role_id}/inline-policies`

```rust
pub fn list_role_inline_policies(&self, role_id: &str, query: &m::ListRoleInlinePoliciesQuery) -> PagedRequest<m::ListRoleInlinePoliciesResponse, m::ListRoleInlinePoliciesItem>
```

## `iam.list_role_policies`

List role policies

`GET /v1/roles/{role_id}/policies`

```rust
pub fn list_role_policies(&self, role_id: &str, query: &m::ListRolePoliciesQuery) -> PagedRequest<m::ListRolePoliciesResponse, m::ListRolePoliciesItem>
```

## `iam.list_roles`

List roles

`GET /v1/roles`

```rust
pub fn list_roles(&self, query: &m::ListRolesQuery) -> PagedRequest<m::ListRolesResponse, m::ListRolesItem>
```

## `iam.list_sts_sessions`

List STS sessions

`GET /v1/sts-sessions`

```rust
pub fn list_sts_sessions(&self, query: &m::ListSTSSessionsQuery) -> PagedRequest<m::ListSTSSessionsResponse, m::ListSTSSessionsItem>
```

## `iam.list_service_account_credentials`

List credentials

`GET /v1/service-accounts/{service_account_id}/credentials`

```rust
pub fn list_service_account_credentials(&self, service_account_id: &str, query: &m::ListServiceAccountCredentialsQuery) -> PagedRequest<m::ListServiceAccountCredentialsResponse, m::ListServiceAccountCredentialsItem>
```

## `iam.list_service_account_inline_policies`

List a service account's inline policies

`GET /v1/service-accounts/{service_account_id}/inline-policies`

```rust
pub fn list_service_account_inline_policies(&self, service_account_id: &str, query: &m::ListServiceAccountInlinePoliciesQuery) -> PagedRequest<m::ListServiceAccountInlinePoliciesResponse, m::ListServiceAccountInlinePoliciesItem>
```

## `iam.list_service_account_policies`

List service account policies

`GET /v1/service-accounts/{service_account_id}/policies`

```rust
pub fn list_service_account_policies(&self, service_account_id: &str, query: &m::ListServiceAccountPoliciesQuery) -> PagedRequest<m::ListServiceAccountPoliciesResponse, m::ListServiceAccountPoliciesItem>
```

## `iam.list_service_account_ssh_keys`

List service-account SSH keys

`GET /v1/service-accounts/{service_account_id}/ssh-keys`

```rust
pub fn list_service_account_ssh_keys(&self, service_account_id: &str) -> PagedRequest<m::ListServiceAccountSSHKeysResponse, m::ListServiceAccountSSHKeysItem>
```

## `iam.list_service_accounts`

List service accounts

`GET /v1/service-accounts`

```rust
pub fn list_service_accounts(&self, query: &m::ListServiceAccountsQuery) -> PagedRequest<m::ListServiceAccountsResponse, m::ListServiceAccountsItem>
```

## `iam.put_role_inline_policy`

Create or replace a role's inline policy

`PUT /v1/roles/{role_id}/inline-policies/{policy_name}`

```rust
pub fn put_role_inline_policy(&self, role_id: &str, policy_name: &str, body: &m::PutRoleInlinePolicyBody) -> Request<m::PutRoleInlinePolicyResponse>
```

## `iam.put_service_account_inline_policy`

Create or replace a service account's inline policy

`PUT /v1/service-accounts/{service_account_id}/inline-policies/{policy_name}`

```rust
pub fn put_service_account_inline_policy(&self, service_account_id: &str, policy_name: &str, body: &m::PutServiceAccountInlinePolicyBody) -> Request<m::PutServiceAccountInlinePolicyResponse>
```

## `iam.remove_role_permission_boundary`

Remove a role's permission boundary

`DELETE /v1/roles/{role_id}/permission-boundary`

```rust
pub fn remove_role_permission_boundary(&self, role_id: &str) -> EmptyRequest
```

## `iam.remove_service_account_permission_boundary`

Remove a service account's permission boundary

`DELETE /v1/service-accounts/{service_account_id}/permission-boundary`

```rust
pub fn remove_service_account_permission_boundary(&self, service_account_id: &str) -> EmptyRequest
```

## `iam.revoke_oauth_token`

Revoke a bearer token

`POST /v1/oauth/revoke`

```rust
pub fn revoke_oauth_token(&self, body: &m::RevokeOAuthTokenBody) -> EmptyRequest
```

## `iam.revoke_sts_session`

Revoke STS session

`DELETE /v1/sts-sessions/{session_id}`

```rust
pub fn revoke_sts_session(&self, session_id: &str, body: Option<&m::RevokeSTSSessionBody>) -> Request<m::RevokeSTSSessionResponse>
```

## `iam.set_role_permission_boundary`

Set a role's permission boundary

`PUT /v1/roles/{role_id}/permission-boundary`

```rust
pub fn set_role_permission_boundary(&self, role_id: &str, body: &m::SetRolePermissionBoundaryBody) -> EmptyRequest
```

## `iam.set_service_account_permission_boundary`

Set a service account's permission boundary

`PUT /v1/service-accounts/{service_account_id}/permission-boundary`

```rust
pub fn set_service_account_permission_boundary(&self, service_account_id: &str, body: &m::SetServiceAccountPermissionBoundaryBody) -> EmptyRequest
```

## `iam.update_oidc_provider`

Update an OIDC provider

`PATCH /v1/oidc-providers/{provider_id}`

```rust
pub fn update_oidc_provider(&self, provider_id: &str, body: &m::UpdateOIDCProviderBody) -> Request<m::UpdateOIDCProviderResponse>
```

## `iam.update_policy`

Update policy

`PATCH /v1/policies/{policy_id}`

```rust
pub fn update_policy(&self, policy_id: &str, body: &m::UpdatePolicyBody) -> Request<m::UpdatePolicyResponse>
```

## `iam.update_role`

Update role

`PATCH /v1/roles/{role_id}`

```rust
pub fn update_role(&self, role_id: &str, body: &m::UpdateRoleBody) -> Request<m::UpdateRoleResponse>
```

## `iam.update_service_account`

Update service account

`PATCH /v1/service-accounts/{service_account_id}`

```rust
pub fn update_service_account(&self, service_account_id: &str, body: &m::UpdateServiceAccountBody) -> Request<m::UpdateServiceAccountResponse>
```

## `kms.cancel_key_deletion`

Cancel a scheduled deletion

`POST /v1/keys/{key_id}/cancel-deletion`

```rust
pub fn cancel_key_deletion(&self, key_id: &str) -> Request<m::CancelKeyDeletionResponse>
```

## `kms.create_key`

Create a KMS key

`POST /v1/keys`

```rust
pub fn create_key(&self, body: &m::CreateKeyBody) -> Request<m::CreateKeyResponse>
```

## `kms.decrypt`

Decrypt a ciphertext

`POST /v1/keys/{key_id}/decrypt`

```rust
pub fn decrypt(&self, key_id: &str, body: &m::DecryptBody) -> Request<m::DecryptResponse>
```

## `kms.disable_key`

Disable a key

`POST /v1/keys/{key_id}/disable`

```rust
pub fn disable_key(&self, key_id: &str) -> Request<m::DisableKeyResponse>
```

## `kms.enable_key`

Enable a disabled key

`POST /v1/keys/{key_id}/enable`

```rust
pub fn enable_key(&self, key_id: &str) -> Request<m::EnableKeyResponse>
```

## `kms.encrypt`

Encrypt a payload

`POST /v1/keys/{key_id}/encrypt`

```rust
pub fn encrypt(&self, key_id: &str, body: &m::EncryptBody) -> Request<m::EncryptResponse>
```

## `kms.generate_data_key`

Generate a fresh data key

`POST /v1/keys/{key_id}/generate-data-key`

```rust
pub fn generate_data_key(&self, key_id: &str, body: Option<&m::GenerateDataKeyBody>) -> Request<m::GenerateDataKeyResponse>
```

## `kms.get_key`

Get a KMS key

`GET /v1/keys/{key_id}`

```rust
pub fn get_key(&self, key_id: &str) -> Request<m::GetKeyResponse>
```

## `kms.get_key_rotation`

Get a key rotation

`GET /v1/keys/{key_id}/rotations/{rotation_id}`

```rust
pub fn get_key_rotation(&self, key_id: &str, rotation_id: &str) -> Request<m::GetKeyRotationResponse>
```

## `kms.get_key_rotation_status`

Get key rotation status

`GET /v1/keys/{key_id}/rotation`

```rust
pub fn get_key_rotation_status(&self, key_id: &str) -> Request<m::GetKeyRotationStatusResponse>
```

## `kms.get_public_key`

Get a versioned public key

`GET /v1/keys/{key_id}/public-key`

```rust
pub fn get_public_key(&self, key_id: &str, query: &m::GetPublicKeyQuery) -> Request<m::GetPublicKeyResponse>
```

## `kms.list_key_rotations`

List key rotation history

`GET /v1/keys/{key_id}/rotations`

```rust
pub fn list_key_rotations(&self, key_id: &str, query: &m::ListKeyRotationsQuery) -> PagedRequest<m::ListKeyRotationsResponse, m::ListKeyRotationsItem>
```

## `kms.list_keys`

List KMS keys

`GET /v1/keys`

```rust
pub fn list_keys(&self, query: &m::ListKeysQuery) -> PagedRequest<m::ListKeysResponse, m::ListKeysItem>
```

## `kms.rewrap`

Rewrap ciphertext with current key material

`POST /v1/keys/{key_id}/rewrap`

```rust
pub fn rewrap(&self, key_id: &str, body: &m::RewrapBody) -> Request<m::RewrapResponse>
```

## `kms.rotate_key_on_demand`

Rotate key material now

`POST /v1/keys/{key_id}/rotate`

```rust
pub fn rotate_key_on_demand(&self, key_id: &str) -> Request<m::RotateKeyOnDemandResponse>
```

## `kms.schedule_key_deletion`

Schedule key for deletion

`POST /v1/keys/{key_id}/schedule-deletion`

```rust
pub fn schedule_key_deletion(&self, key_id: &str, body: Option<&m::ScheduleKeyDeletionBody>) -> Request<m::ScheduleKeyDeletionResponse>
```

## `kms.set_key_rotation`

Configure automatic key rotation

`PUT /v1/keys/{key_id}/rotation`

```rust
pub fn set_key_rotation(&self, key_id: &str, body: &m::SetKeyRotationBody) -> Request<m::SetKeyRotationResponse>
```

## `kms.sign`

Sign a message

`POST /v1/keys/{key_id}/sign`

```rust
pub fn sign(&self, key_id: &str, body: &m::SignBody) -> Request<m::SignResponse>
```

## `kms.update_key`

Update key metadata

`PATCH /v1/keys/{key_id}`

```rust
pub fn update_key(&self, key_id: &str, body: &m::UpdateKeyBody) -> Request<m::UpdateKeyResponse>
```

## `kms.verify`

Verify a signature

`POST /v1/keys/{key_id}/verify`

```rust
pub fn verify(&self, key_id: &str, body: &m::VerifyBody) -> Request<m::VerifyResponse>
```

## `loadbalancer.attach_listener_certificate`

Attach an additional certificate to an HTTPS listener

`POST /v1/load-balancers/{id}/listeners/{listener_id}/certificates`

```rust
pub fn attach_listener_certificate(&self, id: &str, listener_id: &str, body: &m::AttachListenerCertificateBody) -> Request<m::AttachListenerCertificateResponse>
```

## `loadbalancer.attach_target`

Attach a target to this group

`POST /v1/target-groups/{id}/targets`

```rust
pub fn attach_target(&self, id: &str, body: &m::AttachTargetBody) -> Request<m::AttachTargetResponse>
```

## `loadbalancer.create_listener`

Create a listener on this load balancer

`POST /v1/load-balancers/{id}/listeners`

```rust
pub fn create_listener(&self, id: &str, body: &m::CreateListenerBody) -> Request<m::CreateListenerResponse>
```

## `loadbalancer.create_load_balancer`

Create a load balancer

`POST /v1/load-balancers`

```rust
pub fn create_load_balancer(&self, body: &m::CreateLoadBalancerBody) -> Request<m::CreateLoadBalancerResponse>
```

## `loadbalancer.create_rule`

Create a routing rule on this listener (HTTP/HTTPS only)

`POST /v1/load-balancers/{id}/listeners/{listener_id}/rules`

```rust
pub fn create_rule(&self, id: &str, listener_id: &str, body: &m::CreateRuleBody) -> Request<m::CreateRuleResponse>
```

## `loadbalancer.create_target_group`

Create a target group

`POST /v1/target-groups`

```rust
pub fn create_target_group(&self, body: &m::CreateTargetGroupBody) -> Request<m::CreateTargetGroupResponse>
```

## `loadbalancer.delete_listener`

Delete a listener

`DELETE /v1/load-balancers/{id}/listeners/{listener_id}`

```rust
pub fn delete_listener(&self, id: &str, listener_id: &str) -> EmptyRequest
```

## `loadbalancer.delete_load_balancer`

Delete a load balancer

`DELETE /v1/load-balancers/{id}`

```rust
pub fn delete_load_balancer(&self, id: &str) -> EmptyRequest
```

## `loadbalancer.delete_rule_in_listener`

Delete a routing rule

`DELETE /v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}`

```rust
pub fn delete_rule_in_listener(&self, id: &str, listener_id: &str, rule_id: &str) -> EmptyRequest
```

## `loadbalancer.delete_target_group`

Delete a target group

`DELETE /v1/target-groups/{id}`

```rust
pub fn delete_target_group(&self, id: &str) -> EmptyRequest
```

## `loadbalancer.detach_listener_certificate`

Detach a certificate from an HTTPS listener

`DELETE /v1/load-balancers/{id}/listeners/{listener_id}/certificates/{certificate_id}`

```rust
pub fn detach_listener_certificate(&self, id: &str, listener_id: &str, certificate_id: &str) -> EmptyRequest
```

## `loadbalancer.detach_target`

Detach a target

`DELETE /v1/target-groups/{id}/targets/{target_id}`

```rust
pub fn detach_target(&self, id: &str, target_id: &str) -> EmptyRequest
```

## `loadbalancer.get_listener`

Get a listener

`GET /v1/load-balancers/{id}/listeners/{listener_id}`

```rust
pub fn get_listener(&self, id: &str, listener_id: &str) -> Request<m::GetListenerResponse>
```

## `loadbalancer.get_load_balancer`

Get a load balancer

`GET /v1/load-balancers/{id}`

```rust
pub fn get_load_balancer(&self, id: &str) -> Request<m::GetLoadBalancerResponse>
```

## `loadbalancer.get_rule`

Get a routing rule

`GET /v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}`

```rust
pub fn get_rule(&self, id: &str, listener_id: &str, rule_id: &str) -> Request<m::GetRuleResponse>
```

## `loadbalancer.get_target`

Get a target

`GET /v1/target-groups/{id}/targets/{target_id}`

```rust
pub fn get_target(&self, id: &str, target_id: &str) -> Request<m::GetTargetResponse>
```

## `loadbalancer.get_target_group`

Get a target group

`GET /v1/target-groups/{id}`

```rust
pub fn get_target_group(&self, id: &str) -> Request<m::GetTargetGroupResponse>
```

## `loadbalancer.list_listener_certificates`

List listener certificates

`GET /v1/load-balancers/{id}/listeners/{listener_id}/certificates`

```rust
pub fn list_listener_certificates(&self, id: &str, listener_id: &str) -> PagedRequest<m::ListListenerCertificatesResponse, m::ListListenerCertificatesItem>
```

## `loadbalancer.list_listeners`

List this load balancer's listeners

`GET /v1/load-balancers/{id}/listeners`

```rust
pub fn list_listeners(&self, id: &str, query: &m::ListListenersQuery) -> PagedRequest<m::ListListenersResponse, m::ListListenersItem>
```

## `loadbalancer.list_load_balancer_instances`

List a load balancer's instances

`GET /v1/load-balancers/{id}/instances`

```rust
pub fn list_load_balancer_instances(&self, id: &str, query: &m::ListLoadBalancerInstancesQuery) -> PagedRequest<m::ListLoadBalancerInstancesResponse, m::ListLoadBalancerInstancesItem>
```

## `loadbalancer.list_load_balancers`

List load balancers

`GET /v1/load-balancers`

```rust
pub fn list_load_balancers(&self, query: &m::ListLoadBalancersQuery) -> PagedRequest<m::ListLoadBalancersResponse, m::ListLoadBalancersItem>
```

## `loadbalancer.list_rules`

List this listener's rules

`GET /v1/load-balancers/{id}/listeners/{listener_id}/rules`

```rust
pub fn list_rules(&self, id: &str, listener_id: &str, query: &m::ListRulesQuery) -> PagedRequest<m::ListRulesResponse, m::ListRulesItem>
```

## `loadbalancer.list_target_groups`

List target groups

`GET /v1/target-groups`

```rust
pub fn list_target_groups(&self, query: &m::ListTargetGroupsQuery) -> PagedRequest<m::ListTargetGroupsResponse, m::ListTargetGroupsItem>
```

## `loadbalancer.list_targets`

List targets in this group

`GET /v1/target-groups/{id}/targets`

```rust
pub fn list_targets(&self, id: &str, query: &m::ListTargetsQuery) -> PagedRequest<m::ListTargetsResponse, m::ListTargetsItem>
```

## `loadbalancer.update_listener`

Update a listener

`PATCH /v1/load-balancers/{id}/listeners/{listener_id}`

```rust
pub fn update_listener(&self, id: &str, listener_id: &str, body: &m::UpdateListenerBody) -> Request<m::UpdateListenerResponse>
```

## `loadbalancer.update_load_balancer`

Scale or resize a load balancer

`PATCH /v1/load-balancers/{id}`

```rust
pub fn update_load_balancer(&self, id: &str, body: &m::UpdateLoadBalancerBody) -> Request<m::UpdateLoadBalancerResponse>
```

## `loadbalancer.update_rule`

Update a routing rule (full replace)

`PATCH /v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}`

```rust
pub fn update_rule(&self, id: &str, listener_id: &str, rule_id: &str, body: &m::UpdateRuleBody) -> Request<m::UpdateRuleResponse>
```

## `loadbalancer.update_target_group`

Update target group health checks, framing, or stickiness

`PATCH /v1/target-groups/{id}`

```rust
pub fn update_target_group(&self, id: &str, body: &m::UpdateTargetGroupBody) -> Request<m::UpdateTargetGroupResponse>
```

## `network.attach_floating_ip`

Attach an interface to a floating IP

`POST /v1/floating-ips/{floating_ip_id}/interfaces`

```rust
pub fn attach_floating_ip(&self, floating_ip_id: &str, body: &m::AttachFloatingIpBody) -> Request<m::AttachFloatingIpResponse>
```

## `network.attach_interface_security_group`

Attach a security group to an interface

`POST /v1/interfaces/{interface_id}/security-groups`

```rust
pub fn attach_interface_security_group(&self, interface_id: &str, body: &m::AttachInterfaceSecurityGroupBody) -> Request<m::AttachInterfaceSecurityGroupResponse>
```

## `network.create_egress_only_gateway`

Create egress-only gateway

`POST /v1/egress-only-gateways`

```rust
pub fn create_egress_only_gateway(&self, body: &m::CreateEgressOnlyGatewayBody) -> Request<m::CreateEgressOnlyGatewayResponse>
```

## `network.create_floating_ip`

Allocate floating IP

`POST /v1/floating-ips`

```rust
pub fn create_floating_ip(&self, body: Option<&m::CreateFloatingIpBody>) -> Request<m::CreateFloatingIpResponse>
```

## `network.create_interface`

Create interface

`POST /v1/interfaces`

```rust
pub fn create_interface(&self, body: &m::CreateInterfaceBody) -> Request<m::CreateInterfaceResponse>
```

## `network.create_interface_address`

Create interface address

`POST /v1/interfaces/{interface_id}/addresses`

```rust
pub fn create_interface_address(&self, interface_id: &str, body: &m::CreateInterfaceAddressBody) -> Request<m::CreateInterfaceAddressResponse>
```

## `network.create_interface_prefix`

Create interface prefix

`POST /v1/interfaces/{interface_id}/prefixes`

```rust
pub fn create_interface_prefix(&self, interface_id: &str, body: &m::CreateInterfacePrefixBody) -> Request<m::CreateInterfacePrefixResponse>
```

## `network.create_internet_gateway`

Create internet gateway

`POST /v1/internet-gateways`

```rust
pub fn create_internet_gateway(&self, body: &m::CreateInternetGatewayBody) -> Request<m::CreateInternetGatewayResponse>
```

## `network.create_nat_gateway`

Create NAT gateway

`POST /v1/nat-gateways`

```rust
pub fn create_nat_gateway(&self, body: &m::CreateNATGatewayBody) -> Request<m::CreateNATGatewayResponse>
```

## `network.create_prefix_pool`

Create prefix pool

`POST /v1/vpcs/{vpc_id}/prefix-pools`

```rust
pub fn create_prefix_pool(&self, vpc_id: &str, body: &m::CreatePrefixPoolBody) -> Request<m::CreatePrefixPoolResponse>
```

## `network.create_route`

Create route

`POST /v1/route-tables/{route_table_id}/routes`

```rust
pub fn create_route(&self, route_table_id: &str, body: &m::CreateRouteBody) -> Request<m::CreateRouteResponse>
```

## `network.create_route_table`

Create route table

`POST /v1/route-tables`

```rust
pub fn create_route_table(&self, body: &m::CreateRouteTableBody) -> Request<m::CreateRouteTableResponse>
```

## `network.create_security_group`

Create security group

`POST /v1/security-groups`

```rust
pub fn create_security_group(&self, body: &m::CreateSecurityGroupBody) -> Request<m::CreateSecurityGroupResponse>
```

## `network.create_security_group_rule`

Create security group rule

`POST /v1/security-groups/{security_group_id}/rules`

```rust
pub fn create_security_group_rule(&self, security_group_id: &str, body: &m::CreateSecurityGroupRuleBody) -> Request<m::CreateSecurityGroupRuleResponse>
```

## `network.create_subnet`

Create subnet

`POST /v1/subnets`

```rust
pub fn create_subnet(&self, body: &m::CreateSubnetBody) -> Request<m::CreateSubnetResponse>
```

## `network.create_vpc`

Create VPC

`POST /v1/vpcs`

```rust
pub fn create_vpc(&self, body: &m::CreateVpcBody) -> Request<m::CreateVpcResponse>
```

## `network.delete_egress_only_gateway`

Delete egress-only gateway

`DELETE /v1/egress-only-gateways/{egress_only_gateway_id}`

```rust
pub fn delete_egress_only_gateway(&self, egress_only_gateway_id: &str) -> EmptyRequest
```

## `network.delete_floating_ip`

Release floating IP

`DELETE /v1/floating-ips/{floating_ip_id}`

```rust
pub fn delete_floating_ip(&self, floating_ip_id: &str) -> EmptyRequest
```

## `network.delete_interface`

Delete interface

`DELETE /v1/interfaces/{interface_id}`

```rust
pub fn delete_interface(&self, interface_id: &str) -> EmptyRequest
```

## `network.delete_interface_address`

Delete interface address

`DELETE /v1/interfaces/{interface_id}/addresses/{address_id}`

```rust
pub fn delete_interface_address(&self, interface_id: &str, address_id: &str) -> EmptyRequest
```

## `network.delete_interface_prefix`

Delete interface prefix

`DELETE /v1/interfaces/{interface_id}/prefixes/{prefix_id}`

```rust
pub fn delete_interface_prefix(&self, interface_id: &str, prefix_id: &str) -> EmptyRequest
```

## `network.delete_internet_gateway`

Delete internet gateway

`DELETE /v1/internet-gateways/{internet_gateway_id}`

```rust
pub fn delete_internet_gateway(&self, internet_gateway_id: &str) -> EmptyRequest
```

## `network.delete_nat_gateway`

Delete NAT gateway

`DELETE /v1/nat-gateways/{nat_gateway_id}`

```rust
pub fn delete_nat_gateway(&self, nat_gateway_id: &str) -> EmptyRequest
```

## `network.delete_prefix_pool`

Delete prefix pool

`DELETE /v1/vpcs/{vpc_id}/prefix-pools/{pool_id}`

```rust
pub fn delete_prefix_pool(&self, vpc_id: &str, pool_id: &str) -> EmptyRequest
```

## `network.delete_route`

Delete route

`DELETE /v1/route-tables/{route_table_id}/routes/{route_id}`

```rust
pub fn delete_route(&self, route_table_id: &str, route_id: &str) -> EmptyRequest
```

## `network.delete_route_table`

Delete route table

`DELETE /v1/route-tables/{route_table_id}`

```rust
pub fn delete_route_table(&self, route_table_id: &str) -> EmptyRequest
```

## `network.delete_security_group`

Delete security group

`DELETE /v1/security-groups/{security_group_id}`

```rust
pub fn delete_security_group(&self, security_group_id: &str) -> EmptyRequest
```

## `network.delete_security_group_rule`

Delete security group rule

`DELETE /v1/security-groups/{security_group_id}/rules/{rule_id}`

```rust
pub fn delete_security_group_rule(&self, security_group_id: &str, rule_id: &str) -> EmptyRequest
```

## `network.delete_subnet`

Delete subnet

`DELETE /v1/subnets/{subnet_id}`

```rust
pub fn delete_subnet(&self, subnet_id: &str) -> EmptyRequest
```

## `network.delete_vpc`

Delete VPC

`DELETE /v1/vpcs/{vpc_id}`

```rust
pub fn delete_vpc(&self, vpc_id: &str) -> EmptyRequest
```

## `network.detach_floating_ip`

Detach all direct interfaces from a floating IP

`DELETE /v1/floating-ips/{floating_ip_id}/interfaces`

```rust
pub fn detach_floating_ip(&self, floating_ip_id: &str) -> EmptyRequest
```

## `network.detach_floating_ip_interface`

Detach an interface from a floating IP

`DELETE /v1/floating-ips/{floating_ip_id}/interfaces/{interface_id}`

```rust
pub fn detach_floating_ip_interface(&self, floating_ip_id: &str, interface_id: &str) -> EmptyRequest
```

## `network.detach_interface_security_group`

Detach a security group from an interface

`DELETE /v1/interfaces/{interface_id}/security-groups/{security_group_id}`

```rust
pub fn detach_interface_security_group(&self, interface_id: &str, security_group_id: &str) -> EmptyRequest
```

## `network.enable_subnet_i_pv6`

Enable IPv6 on a subnet

`POST /v1/subnets/{subnet_id}/ipv6`

```rust
pub fn enable_subnet_i_pv6(&self, subnet_id: &str, body: &m::EnableSubnetIPv6Body) -> Request<m::EnableSubnetIPv6Response>
```

## `network.enable_vpci_pv6`

Enable IPv6 on a vpc

`POST /v1/vpcs/{vpc_id}/ipv6`

```rust
pub fn enable_vpci_pv6(&self, vpc_id: &str, body: &m::EnableVPCIPv6Body) -> Request<m::EnableVPCIPv6Response>
```

## `network.get_egress_only_gateway`

Get egress-only gateway

`GET /v1/egress-only-gateways/{egress_only_gateway_id}`

```rust
pub fn get_egress_only_gateway(&self, egress_only_gateway_id: &str) -> Request<m::GetEgressOnlyGatewayResponse>
```

## `network.get_floating_ip`

Get floating IP

`GET /v1/floating-ips/{floating_ip_id}`

```rust
pub fn get_floating_ip(&self, floating_ip_id: &str) -> Request<m::GetFloatingIpResponse>
```

## `network.get_interface`

Get interface

`GET /v1/interfaces/{interface_id}`

```rust
pub fn get_interface(&self, interface_id: &str) -> Request<m::GetInterfaceResponse>
```

## `network.get_interface_address`

Get interface address

`GET /v1/interfaces/{interface_id}/addresses/{address_id}`

```rust
pub fn get_interface_address(&self, interface_id: &str, address_id: &str) -> Request<m::GetInterfaceAddressResponse>
```

## `network.get_internet_gateway`

Get internet gateway

`GET /v1/internet-gateways/{internet_gateway_id}`

```rust
pub fn get_internet_gateway(&self, internet_gateway_id: &str) -> Request<m::GetInternetGatewayResponse>
```

## `network.get_nat_gateway`

Get NAT gateway

`GET /v1/nat-gateways/{nat_gateway_id}`

```rust
pub fn get_nat_gateway(&self, nat_gateway_id: &str) -> Request<m::GetNATGatewayResponse>
```

## `network.get_route`

Get route

`GET /v1/route-tables/{route_table_id}/routes/{route_id}`

```rust
pub fn get_route(&self, route_table_id: &str, route_id: &str) -> Request<m::GetRouteResponse>
```

## `network.get_route_table`

Get route table

`GET /v1/route-tables/{route_table_id}`

```rust
pub fn get_route_table(&self, route_table_id: &str) -> Request<m::GetRouteTableResponse>
```

## `network.get_security_group`

Get security group

`GET /v1/security-groups/{security_group_id}`

```rust
pub fn get_security_group(&self, security_group_id: &str) -> Request<m::GetSecurityGroupResponse>
```

## `network.get_security_group_rule`

Get security group rule

`GET /v1/security-groups/{security_group_id}/rules/{rule_id}`

```rust
pub fn get_security_group_rule(&self, security_group_id: &str, rule_id: &str) -> Request<m::GetSecurityGroupRuleResponse>
```

## `network.get_subnet`

Get subnet

`GET /v1/subnets/{subnet_id}`

```rust
pub fn get_subnet(&self, subnet_id: &str) -> Request<m::GetSubnetResponse>
```

## `network.get_vpc`

Get VPC

`GET /v1/vpcs/{vpc_id}`

```rust
pub fn get_vpc(&self, vpc_id: &str) -> Request<m::GetVpcResponse>
```

## `network.list_egress_only_gateway_routes`

List egress-only gateway routes

`GET /v1/egress-only-gateways/{egress_only_gateway_id}/routes`

```rust
pub fn list_egress_only_gateway_routes(&self, egress_only_gateway_id: &str, query: &m::ListEgressOnlyGatewayRoutesQuery) -> PagedRequest<m::ListEgressOnlyGatewayRoutesResponse, m::ListEgressOnlyGatewayRoutesItem>
```

## `network.list_egress_only_gateways`

List egress-only gateways

`GET /v1/egress-only-gateways`

```rust
pub fn list_egress_only_gateways(&self, query: &m::ListEgressOnlyGatewaysQuery) -> PagedRequest<m::ListEgressOnlyGatewaysResponse, m::ListEgressOnlyGatewaysItem>
```

## `network.list_floating_ip_interfaces`

List a floating IP’s interfaces

`GET /v1/floating-ips/{floating_ip_id}/interfaces`

```rust
pub fn list_floating_ip_interfaces(&self, floating_ip_id: &str, query: &m::ListFloatingIpInterfacesQuery) -> PagedRequest<m::ListFloatingIpInterfacesResponse, m::ListFloatingIpInterfacesItem>
```

## `network.list_floating_ips`

List floating IPs

`GET /v1/floating-ips`

```rust
pub fn list_floating_ips(&self, query: &m::ListFloatingIpsQuery) -> PagedRequest<m::ListFloatingIpsResponse, m::ListFloatingIpsItem>
```

## `network.list_interface_addresses`

List interface addresses

`GET /v1/interfaces/{interface_id}/addresses`

```rust
pub fn list_interface_addresses(&self, interface_id: &str) -> PagedRequest<m::ListInterfaceAddressesResponse, m::ListInterfaceAddressesItem>
```

## `network.list_interface_prefixes`

List interface prefixes

`GET /v1/interfaces/{interface_id}/prefixes`

```rust
pub fn list_interface_prefixes(&self, interface_id: &str) -> PagedRequest<m::ListInterfacePrefixesResponse, m::ListInterfacePrefixesItem>
```

## `network.list_interface_security_groups`

List an interface’s security groups

`GET /v1/interfaces/{interface_id}/security-groups`

```rust
pub fn list_interface_security_groups(&self, interface_id: &str, query: &m::ListInterfaceSecurityGroupsQuery) -> PagedRequest<m::ListInterfaceSecurityGroupsResponse, m::ListInterfaceSecurityGroupsItem>
```

## `network.list_interfaces`

List interfaces

`GET /v1/interfaces`

```rust
pub fn list_interfaces(&self, query: &m::ListInterfacesQuery) -> PagedRequest<m::ListInterfacesResponse, m::ListInterfacesItem>
```

## `network.list_internet_gateway_routes`

List internet gateway routes

`GET /v1/internet-gateways/{internet_gateway_id}/routes`

```rust
pub fn list_internet_gateway_routes(&self, internet_gateway_id: &str, query: &m::ListInternetGatewayRoutesQuery) -> PagedRequest<m::ListInternetGatewayRoutesResponse, m::ListInternetGatewayRoutesItem>
```

## `network.list_internet_gateways`

List internet gateways

`GET /v1/internet-gateways`

```rust
pub fn list_internet_gateways(&self, query: &m::ListInternetGatewaysQuery) -> PagedRequest<m::ListInternetGatewaysResponse, m::ListInternetGatewaysItem>
```

## `network.list_nat_gateway_routes`

List NAT gateway routes

`GET /v1/nat-gateways/{nat_gateway_id}/routes`

```rust
pub fn list_nat_gateway_routes(&self, nat_gateway_id: &str, query: &m::ListNATGatewayRoutesQuery) -> PagedRequest<m::ListNATGatewayRoutesResponse, m::ListNATGatewayRoutesItem>
```

## `network.list_nat_gateways`

List NAT gateways

`GET /v1/nat-gateways`

```rust
pub fn list_nat_gateways(&self, query: &m::ListNATGatewaysQuery) -> PagedRequest<m::ListNATGatewaysResponse, m::ListNATGatewaysItem>
```

## `network.list_prefix_pools`

List prefix pools

`GET /v1/vpcs/{vpc_id}/prefix-pools`

```rust
pub fn list_prefix_pools(&self, vpc_id: &str) -> PagedRequest<m::ListPrefixPoolsResponse, m::ListPrefixPoolsItem>
```

## `network.list_route_tables`

List route tables

`GET /v1/route-tables`

```rust
pub fn list_route_tables(&self, query: &m::ListRouteTablesQuery) -> PagedRequest<m::ListRouteTablesResponse, m::ListRouteTablesItem>
```

## `network.list_routes`

List routes

`GET /v1/route-tables/{route_table_id}/routes`

```rust
pub fn list_routes(&self, route_table_id: &str, query: &m::ListRoutesQuery) -> PagedRequest<m::ListRoutesResponse, m::ListRoutesItem>
```

## `network.list_security_group_rules`

List security group rules

`GET /v1/security-groups/{security_group_id}/rules`

```rust
pub fn list_security_group_rules(&self, security_group_id: &str, query: &m::ListSecurityGroupRulesQuery) -> PagedRequest<m::ListSecurityGroupRulesResponse, m::ListSecurityGroupRulesItem>
```

## `network.list_security_groups`

List security groups

`GET /v1/security-groups`

```rust
pub fn list_security_groups(&self, query: &m::ListSecurityGroupsQuery) -> PagedRequest<m::ListSecurityGroupsResponse, m::ListSecurityGroupsItem>
```

## `network.list_subnets`

List subnets

`GET /v1/subnets`

```rust
pub fn list_subnets(&self, query: &m::ListSubnetsQuery) -> PagedRequest<m::ListSubnetsResponse, m::ListSubnetsItem>
```

## `network.list_vpcs`

List VPCs

`GET /v1/vpcs`

```rust
pub fn list_vpcs(&self, query: &m::ListVpcsQuery) -> PagedRequest<m::ListVpcsResponse, m::ListVpcsItem>
```

## `network.update_egress_only_gateway`

Update egress-only gateway

`PATCH /v1/egress-only-gateways/{egress_only_gateway_id}`

```rust
pub fn update_egress_only_gateway(&self, egress_only_gateway_id: &str, body: &m::UpdateEgressOnlyGatewayBody) -> Request<m::UpdateEgressOnlyGatewayResponse>
```

## `network.update_floating_ip`

Update floating IP

`PATCH /v1/floating-ips/{floating_ip_id}`

```rust
pub fn update_floating_ip(&self, floating_ip_id: &str, body: &m::UpdateFloatingIpBody) -> Request<m::UpdateFloatingIpResponse>
```

## `network.update_interface`

Update interface

`PATCH /v1/interfaces/{interface_id}`

```rust
pub fn update_interface(&self, interface_id: &str, body: &m::UpdateInterfaceBody) -> Request<m::UpdateInterfaceResponse>
```

## `network.update_internet_gateway`

Update internet gateway

`PATCH /v1/internet-gateways/{internet_gateway_id}`

```rust
pub fn update_internet_gateway(&self, internet_gateway_id: &str, body: &m::UpdateInternetGatewayBody) -> Request<m::UpdateInternetGatewayResponse>
```

## `network.update_nat_gateway`

Update NAT gateway

`PATCH /v1/nat-gateways/{nat_gateway_id}`

```rust
pub fn update_nat_gateway(&self, nat_gateway_id: &str, body: &m::UpdateNATGatewayBody) -> Request<m::UpdateNATGatewayResponse>
```

## `network.update_route`

Update route

`PATCH /v1/route-tables/{route_table_id}/routes/{route_id}`

```rust
pub fn update_route(&self, route_table_id: &str, route_id: &str, body: &m::UpdateRouteBody) -> Request<m::UpdateRouteResponse>
```

## `network.update_route_table`

Update route table

`PATCH /v1/route-tables/{route_table_id}`

```rust
pub fn update_route_table(&self, route_table_id: &str, body: &m::UpdateRouteTableBody) -> Request<m::UpdateRouteTableResponse>
```

## `network.update_security_group`

Update security group

`PATCH /v1/security-groups/{security_group_id}`

```rust
pub fn update_security_group(&self, security_group_id: &str, body: &m::UpdateSecurityGroupBody) -> Request<m::UpdateSecurityGroupResponse>
```

## `network.update_subnet`

Update subnet

`PATCH /v1/subnets/{subnet_id}`

```rust
pub fn update_subnet(&self, subnet_id: &str, body: &m::UpdateSubnetBody) -> Request<m::UpdateSubnetResponse>
```

## `network.update_vpc`

Update VPC

`PATCH /v1/vpcs/{vpc_id}`

```rust
pub fn update_vpc(&self, vpc_id: &str, body: &m::UpdateVpcBody) -> Request<m::UpdateVpcResponse>
```

## `notifications.create_topic`

Create a topic

`POST /v1/topics`

```rust
pub fn create_topic(&self, body: &m::CreateTopicBody) -> Request<m::CreateTopicResponse>
```

## `notifications.delete_topic`

Delete a topic

`DELETE /v1/topics/{topic_id}`

```rust
pub fn delete_topic(&self, topic_id: &str) -> EmptyRequest
```

## `notifications.delete_topic_policy`

Delete the topic's resource policy

`DELETE /v1/topics/{topic_id}/policy`

```rust
pub fn delete_topic_policy(&self, topic_id: &str) -> EmptyRequest
```

## `notifications.describe_message`

Describe a published notification

`GET /v1/topics/{topic_id}/messages/{message_id}`

```rust
pub fn describe_message(&self, topic_id: &str, message_id: &str) -> Request<m::DescribeMessageResponse>
```

## `notifications.describe_subscription`

Describe a subscription

`GET /v1/topics/{topic_id}/subscriptions/{subscription_id}`

```rust
pub fn describe_subscription(&self, topic_id: &str, subscription_id: &str) -> Request<m::DescribeSubscriptionResponse>
```

## `notifications.describe_topic`

Describe a topic

`GET /v1/topics/{topic_id}`

```rust
pub fn describe_topic(&self, topic_id: &str) -> Request<m::DescribeTopicResponse>
```

## `notifications.get_subscription_signing_secret`

Read the webhook signing secret

`GET /v1/topics/{topic_id}/subscriptions/{subscription_id}/signing-secret`

```rust
pub fn get_subscription_signing_secret(&self, topic_id: &str, subscription_id: &str) -> Request<m::GetSubscriptionSigningSecretResponse>
```

## `notifications.get_topic_policy`

Read the topic's resource policy

`GET /v1/topics/{topic_id}/policy`

```rust
pub fn get_topic_policy(&self, topic_id: &str) -> Request<m::GetTopicPolicyResponse>
```

## `notifications.list_deliveries`

List delivery records

`GET /v1/topics/{topic_id}/deliveries`

```rust
pub fn list_deliveries(&self, topic_id: &str, query: &m::ListDeliveriesQuery) -> PagedRequest<m::ListDeliveriesResponse, m::ListDeliveriesItem>
```

## `notifications.list_messages`

List messages

`GET /v1/topics/{topic_id}/messages`

```rust
pub fn list_messages(&self, topic_id: &str, query: &m::ListMessagesQuery) -> PagedRequest<m::ListMessagesResponse, m::ListMessagesItem>
```

## `notifications.list_subscriptions`

List subscriptions

`GET /v1/topics/{topic_id}/subscriptions`

```rust
pub fn list_subscriptions(&self, topic_id: &str, query: &m::ListSubscriptionsQuery) -> PagedRequest<m::ListSubscriptionsResponse, m::ListSubscriptionsItem>
```

## `notifications.list_topics`

List topics

`GET /v1/topics`

```rust
pub fn list_topics(&self, query: &m::ListTopicsQuery) -> PagedRequest<m::ListTopicsResponse, m::ListTopicsItem>
```

## `notifications.publish`

Publish a notification

`POST /v1/topics/{topic_id}/publish`

```rust
pub fn publish(&self, topic_id: &str, body: &m::PublishBody) -> Request<m::PublishResponse>
```

## `notifications.publish_batch`

Publish up to 10 notifications

`POST /v1/topics/{topic_id}/publish-batch`

```rust
pub fn publish_batch(&self, topic_id: &str, body: &m::PublishBatchBody) -> Request<m::PublishBatchResponse>
```

## `notifications.put_topic_policy`

Set the topic's resource policy

`PUT /v1/topics/{topic_id}/policy`

```rust
pub fn put_topic_policy(&self, topic_id: &str, body: &m::PutTopicPolicyBody) -> Request<m::PutTopicPolicyResponse>
```

## `notifications.resend_confirmation`

Re-send the confirmation handshake

`POST /v1/topics/{topic_id}/subscriptions/{subscription_id}/resend-confirmation`

```rust
pub fn resend_confirmation(&self, topic_id: &str, subscription_id: &str) -> Request<m::ResendConfirmationResponse>
```

## `notifications.rotate_subscription_signing_secret`

Rotate the webhook signing secret

`POST /v1/topics/{topic_id}/subscriptions/{subscription_id}/signing-secret/rotate`

```rust
pub fn rotate_subscription_signing_secret(&self, topic_id: &str, subscription_id: &str) -> Request<m::RotateSubscriptionSigningSecretResponse>
```

## `notifications.subscribe`

Subscribe an endpoint to a topic

`POST /v1/topics/{topic_id}/subscriptions`

```rust
pub fn subscribe(&self, topic_id: &str, body: &m::SubscribeBody) -> Request<m::SubscribeResponse>
```

## `notifications.unsubscribe`

Unsubscribe

`DELETE /v1/topics/{topic_id}/subscriptions/{subscription_id}`

```rust
pub fn unsubscribe(&self, topic_id: &str, subscription_id: &str) -> EmptyRequest
```

## `notifications.update_subscription`

Update mutable subscription settings

`PATCH /v1/topics/{topic_id}/subscriptions/{subscription_id}`

```rust
pub fn update_subscription(&self, topic_id: &str, subscription_id: &str, body: &m::UpdateSubscriptionBody) -> Request<m::UpdateSubscriptionResponse>
```

## `notifications.update_topic`

Update mutable topic metadata

`PATCH /v1/topics/{topic_id}`

```rust
pub fn update_topic(&self, topic_id: &str, body: &m::UpdateTopicBody) -> Request<m::UpdateTopicResponse>
```

## `queue.change_message_visibility`

Extend or shorten a lease

`POST /v1/queues/{queue_id}/messages/visibility`

```rust
pub fn change_message_visibility(&self, queue_id: &str, body: &m::ChangeMessageVisibilityBody) -> EmptyRequest
```

## `queue.change_message_visibility_batch`

Extend or shorten up to ten leases

`POST /v1/queues/{queue_id}/messages/visibility-batch`

```rust
pub fn change_message_visibility_batch(&self, queue_id: &str, body: &m::ChangeMessageVisibilityBatchBody) -> Request<m::ChangeMessageVisibilityBatchResponse>
```

## `queue.create_queue`

Create a queue

`POST /v1/queues`

```rust
pub fn create_queue(&self, body: &m::CreateQueueBody) -> Request<m::CreateQueueResponse>
```

## `queue.delete_message`

Delete a leased message

`DELETE /v1/queues/{queue_id}/messages`

```rust
pub fn delete_message(&self, queue_id: &str, query: &m::DeleteMessageQuery) -> EmptyRequest
```

## `queue.delete_message_batch`

Delete up to ten leased messages

`POST /v1/queues/{queue_id}/messages/delete-batch`

```rust
pub fn delete_message_batch(&self, queue_id: &str, body: &m::DeleteMessageBatchBody) -> Request<m::DeleteMessageBatchResponse>
```

## `queue.delete_queue`

Delete a queue

`DELETE /v1/queues/{queue_id}`

```rust
pub fn delete_queue(&self, queue_id: &str) -> EmptyRequest
```

## `queue.describe_queue`

Describe a queue

`GET /v1/queues/{queue_id}`

```rust
pub fn describe_queue(&self, queue_id: &str) -> Request<m::DescribeQueueResponse>
```

## `queue.list_queues`

List queues

`GET /v1/queues`

```rust
pub fn list_queues(&self, query: &m::ListQueuesQuery) -> PagedRequest<m::ListQueuesResponse, m::ListQueuesItem>
```

## `queue.peek_messages`

Peek at a queue's messages

`GET /v1/queues/{queue_id}/messages`

```rust
pub fn peek_messages(&self, queue_id: &str, query: &m::PeekMessagesQuery) -> Request<m::PeekMessagesResponse>
```

## `queue.purge_queue`

Delete every message on a queue

`POST /v1/queues/{queue_id}/purge`

```rust
pub fn purge_queue(&self, queue_id: &str) -> Request<m::PurgeQueueResponse>
```

## `queue.receive_messages`

Lease messages

`POST /v1/queues/{queue_id}/messages/receive`

```rust
pub fn receive_messages(&self, queue_id: &str, body: Option<&m::ReceiveMessagesBody>) -> Request<m::ReceiveMessagesResponse>
```

## `queue.send_message`

Send a message

`POST /v1/queues/{queue_id}/messages`

```rust
pub fn send_message(&self, queue_id: &str, body: &m::SendMessageBody) -> Request<m::SendMessageResponse>
```

## `queue.send_message_batch`

Send up to ten messages

`POST /v1/queues/{queue_id}/messages/batch`

```rust
pub fn send_message_batch(&self, queue_id: &str, body: &m::SendMessageBatchBody) -> Request<m::SendMessageBatchResponse>
```

## `queue.update_queue`

Update queue attributes

`PATCH /v1/queues/{queue_id}`

```rust
pub fn update_queue(&self, queue_id: &str, body: &m::UpdateQueueBody) -> Request<m::UpdateQueueResponse>
```

## `quota.list_quotas`

List quotas

`GET /v1/quotas`

```rust
pub fn list_quotas(&self, query: &m::ListQuotasQuery) -> Request<m::ListQuotasResponse>
```

## `registry.batch_delete_image`

Delete images

`POST /v1/repositories/{repository_id}/images/batch-delete`

```rust
pub fn batch_delete_image(&self, repository_id: &str, body: &m::BatchDeleteImageBody) -> Request<m::BatchDeleteImageResponse>
```

## `registry.create_pull_through_cache_rule`

Create a pull-through cache rule

`POST /v1/pull-through-cache-rules`

```rust
pub fn create_pull_through_cache_rule(&self, body: &m::CreatePullThroughCacheRuleBody) -> Request<m::CreatePullThroughCacheRuleResponse>
```

## `registry.create_repository`

Create a repository

`POST /v1/repositories`

```rust
pub fn create_repository(&self, body: &m::CreateRepositoryBody) -> Request<m::CreateRepositoryResponse>
```

## `registry.delete_encryption_configuration`

Delete encryption configuration

`DELETE /v1/encryption-configuration`

```rust
pub fn delete_encryption_configuration(&self) -> EmptyRequest
```

## `registry.delete_lifecycle_policy`

Delete the lifecycle policy

`DELETE /v1/repositories/{repository_id}/lifecycle-policy`

```rust
pub fn delete_lifecycle_policy(&self, repository_id: &str) -> EmptyRequest
```

## `registry.delete_pull_through_cache_rule`

Delete a pull-through cache rule

`DELETE /v1/pull-through-cache-rules/{prefix}`

```rust
pub fn delete_pull_through_cache_rule(&self, prefix: &str) -> EmptyRequest
```

## `registry.delete_repository`

Delete a repository

`DELETE /v1/repositories/{repository_id}`

```rust
pub fn delete_repository(&self, repository_id: &str, query: &m::DeleteRepositoryQuery) -> EmptyRequest
```

## `registry.delete_repository_policy`

Delete the repository policy

`DELETE /v1/repositories/{repository_id}/policy`

```rust
pub fn delete_repository_policy(&self, repository_id: &str) -> EmptyRequest
```

## `registry.describe_image`

Describe an image

`GET /v1/repositories/{repository_id}/images/{image_reference}`

```rust
pub fn describe_image(&self, repository_id: &str, image_reference: &str) -> Request<m::DescribeImageResponse>
```

## `registry.describe_images`

List images

`GET /v1/repositories/{repository_id}/images`

```rust
pub fn describe_images(&self, repository_id: &str, query: &m::DescribeImagesQuery) -> Request<m::DescribeImagesResponse>
```

## `registry.describe_pull_through_cache_rule`

Get a pull-through cache rule

`GET /v1/pull-through-cache-rules/{prefix}`

```rust
pub fn describe_pull_through_cache_rule(&self, prefix: &str) -> Request<m::DescribePullThroughCacheRuleResponse>
```

## `registry.describe_pull_through_cache_rules`

List pull-through cache rules

`GET /v1/pull-through-cache-rules`

```rust
pub fn describe_pull_through_cache_rules(&self, query: &m::DescribePullThroughCacheRulesQuery) -> Request<m::DescribePullThroughCacheRulesResponse>
```

## `registry.describe_repositories`

List repositories

`GET /v1/repositories`

```rust
pub fn describe_repositories(&self, query: &m::DescribeRepositoriesQuery) -> Request<m::DescribeRepositoriesResponse>
```

## `registry.describe_repository`

Describe a repository

`GET /v1/repositories/{repository_id}`

```rust
pub fn describe_repository(&self, repository_id: &str) -> Request<m::DescribeRepositoryResponse>
```

## `registry.get_authorization_token`

Get a docker login credential

`POST /v1/authorization-token`

```rust
pub fn get_authorization_token(&self) -> Request<m::GetAuthorizationTokenResponse>
```

## `registry.get_encryption_configuration`

Get the encryption configuration

`GET /v1/encryption-configuration`

```rust
pub fn get_encryption_configuration(&self) -> Request<m::GetEncryptionConfigurationResponse>
```

## `registry.get_lifecycle_policy`

Get the lifecycle policy

`GET /v1/repositories/{repository_id}/lifecycle-policy`

```rust
pub fn get_lifecycle_policy(&self, repository_id: &str) -> Request<m::GetLifecyclePolicyResponse>
```

## `registry.get_repository_policy`

Get the repository policy

`GET /v1/repositories/{repository_id}/policy`

```rust
pub fn get_repository_policy(&self, repository_id: &str) -> Request<m::GetRepositoryPolicyResponse>
```

## `registry.get_retained_registry_presence`

Check retained registry data presence

`GET /v1/encryption-configuration/retained-data`

```rust
pub fn get_retained_registry_presence(&self) -> Request<m::GetRetainedRegistryPresenceResponse>
```

## `registry.list_tags`

List tags

`GET /v1/repositories/{repository_id}/tags`

```rust
pub fn list_tags(&self, repository_id: &str, query: &m::ListTagsQuery) -> PagedRequest<m::ListTagsResponse, m::ListTagsItem>
```

## `registry.put_encryption_configuration`

Set the encryption configuration

`PUT /v1/encryption-configuration`

```rust
pub fn put_encryption_configuration(&self, body: &m::PutEncryptionConfigurationBody) -> Request<m::PutEncryptionConfigurationResponse>
```

## `registry.put_lifecycle_policy`

Set the lifecycle policy

`PUT /v1/repositories/{repository_id}/lifecycle-policy`

```rust
pub fn put_lifecycle_policy(&self, repository_id: &str, body: &m::PutLifecyclePolicyBody) -> Request<m::PutLifecyclePolicyResponse>
```

## `registry.set_repository_policy`

Set the repository policy

`PUT /v1/repositories/{repository_id}/policy`

```rust
pub fn set_repository_policy(&self, repository_id: &str, body: &m::SetRepositoryPolicyBody) -> Request<m::SetRepositoryPolicyResponse>
```

## `registry.start_lifecycle_policy_preview`

Preview a lifecycle policy

`POST /v1/repositories/{repository_id}/lifecycle-policy/preview`

```rust
pub fn start_lifecycle_policy_preview(&self, repository_id: &str, body: Option<&m::StartLifecyclePolicyPreviewBody>) -> Request<m::StartLifecyclePolicyPreviewResponse>
```

## `registry.update_pull_through_cache_rule`

Update a pull-through cache rule

`PATCH /v1/pull-through-cache-rules/{prefix}`

```rust
pub fn update_pull_through_cache_rule(&self, prefix: &str, body: &m::UpdatePullThroughCacheRuleBody) -> Request<m::UpdatePullThroughCacheRuleResponse>
```

## `registry.update_repository`

Update repository settings

`PATCH /v1/repositories/{repository_id}`

```rust
pub fn update_repository(&self, repository_id: &str, body: &m::UpdateRepositoryBody) -> Request<m::UpdateRepositoryResponse>
```

## `secrets.create_secret`

Create a new secret with an initial value

`POST /v1/secrets`

```rust
pub fn create_secret(&self, body: &m::CreateSecretBody) -> Request<m::CreateSecretResponse>
```

## `secrets.delete_secret`

Schedule deletion (soft delete with recovery window)

`DELETE /v1/secrets/{secret_id}`

```rust
pub fn delete_secret(&self, secret_id: &str, body: Option<&m::DeleteSecretBody>) -> Request<m::DeleteSecretResponse>
```

## `secrets.describe_secret`

Describe a secret (no value)

`GET /v1/secrets/{secret_id}`

```rust
pub fn describe_secret(&self, secret_id: &str) -> Request<m::DescribeSecretResponse>
```

## `secrets.get_secret_value`

Read the current value (or a specific version)

`GET /v1/secrets/{secret_id}/value`

```rust
pub fn get_secret_value(&self, secret_id: &str, query: &m::GetSecretValueQuery) -> Request<m::GetSecretValueResponse>
```

## `secrets.list_secrets`

List secrets

`GET /v1/secrets`

```rust
pub fn list_secrets(&self, query: &m::ListSecretsQuery) -> PagedRequest<m::ListSecretsResponse, m::ListSecretsItem>
```

## `secrets.list_versions`

List versions

`GET /v1/secrets/{secret_id}/versions`

```rust
pub fn list_versions(&self, secret_id: &str, query: &m::ListVersionsQuery) -> PagedRequest<m::ListVersionsResponse, m::ListVersionsItem>
```

## `secrets.put_secret_value`

Store a new version (becomes current)

`POST /v1/secrets/{secret_id}/value`

```rust
pub fn put_secret_value(&self, secret_id: &str, body: &m::PutSecretValueBody) -> Request<m::PutSecretValueResponse>
```

## `secrets.restore_secret`

Restore a secret from the recovery window

`POST /v1/secrets/{secret_id}/restore`

```rust
pub fn restore_secret(&self, secret_id: &str) -> Request<m::RestoreSecretResponse>
```

## `secrets.update_secret`

Update mutable metadata

`PATCH /v1/secrets/{secret_id}`

```rust
pub fn update_secret(&self, secret_id: &str, body: &m::UpdateSecretBody) -> Request<m::UpdateSecretResponse>
```

## `storage.abort_multipart_upload`

Abort a multipart upload

`DELETE /v1/buckets/{bucket}/multipart-uploads/{upload_id}`

```rust
pub fn abort_multipart_upload(&self, bucket: &str, upload_id: &str) -> EmptyRequest
```

## `storage.complete_multipart_upload`

Complete a multipart upload

`POST /v1/buckets/{bucket}/multipart-uploads/{upload_id}/complete`

```rust
pub fn complete_multipart_upload(&self, bucket: &str, upload_id: &str, body: &m::CompleteMultipartUploadBody) -> Request<m::CompleteMultipartUploadResponse>
```

## `storage.create_bucket`

Create bucket

`POST /v1/buckets`

```rust
pub fn create_bucket(&self, body: &m::CreateBucketBody) -> Request<m::CreateBucketResponse>
```

## `storage.create_snapshot`

Create snapshot

`POST /v1/snapshots`

```rust
pub fn create_snapshot(&self, body: &m::CreateSnapshotBody) -> Request<m::CreateSnapshotResponse>
```

## `storage.create_snapshot_policy`

Create snapshot policy

`POST /v1/volumes/{volume_id}/snapshot-policies`

```rust
pub fn create_snapshot_policy(&self, volume_id: &str, body: &m::CreateSnapshotPolicyBody) -> Request<m::CreateSnapshotPolicyResponse>
```

## `storage.create_volume`

Create volume

`POST /v1/volumes`

```rust
pub fn create_volume(&self, body: &m::CreateVolumeBody) -> Request<m::CreateVolumeResponse>
```

## `storage.delete_bucket`

Delete bucket

`DELETE /v1/buckets/{bucket}`

```rust
pub fn delete_bucket(&self, bucket: &str) -> Request<m::DeleteBucketResponse>
```

## `storage.delete_bucket_cors`

Delete bucket CORS configuration

`DELETE /v1/buckets/{bucket}/cors`

```rust
pub fn delete_bucket_cors(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_bucket_encryption`

Delete bucket encryption configuration

`DELETE /v1/buckets/{bucket}/encryption`

```rust
pub fn delete_bucket_encryption(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_bucket_lifecycle`

Delete bucket lifecycle configuration

`DELETE /v1/buckets/{bucket}/lifecycle`

```rust
pub fn delete_bucket_lifecycle(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_bucket_object_lock`

Delete bucket object-lock configuration

`DELETE /v1/buckets/{bucket}/object-lock`

```rust
pub fn delete_bucket_object_lock(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_bucket_policy`

Delete bucket policy

`DELETE /v1/buckets/{bucket}/policy`

```rust
pub fn delete_bucket_policy(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_bucket_tagging`

Delete bucket tag set

`DELETE /v1/buckets/{bucket}/tagging`

```rust
pub fn delete_bucket_tagging(&self, bucket: &str) -> EmptyRequest
```

## `storage.delete_object`

Delete object

`DELETE /v1/buckets/{bucket}/objects/{key}`

```rust
pub fn delete_object(&self, bucket: &str, key: &str) -> EmptyRequest
```

## `storage.delete_snapshot`

Delete snapshot

`DELETE /v1/snapshots/{snapshot_id}`

```rust
pub fn delete_snapshot(&self, snapshot_id: &str) -> EmptyRequest
```

## `storage.delete_snapshot_policy`

Delete snapshot policy

`DELETE /v1/volumes/{volume_id}/snapshot-policies/{policy_id}`

```rust
pub fn delete_snapshot_policy(&self, volume_id: &str, policy_id: &str) -> EmptyRequest
```

## `storage.delete_volume`

Delete volume

`DELETE /v1/volumes/{volume_id}`

```rust
pub fn delete_volume(&self, volume_id: &str) -> EmptyRequest
```

## `storage.extend_volume`

Extend volume

`POST /v1/volumes/{volume_id}/extend`

```rust
pub fn extend_volume(&self, volume_id: &str, body: &m::ExtendVolumeBody) -> Request<m::ExtendVolumeResponse>
```

## `storage.get_bucket_cors`

Get bucket CORS configuration

`GET /v1/buckets/{bucket}/cors`

```rust
pub fn get_bucket_cors(&self, bucket: &str) -> Request<m::GetBucketCORSResponse>
```

## `storage.get_bucket_encryption`

Get bucket encryption configuration

`GET /v1/buckets/{bucket}/encryption`

```rust
pub fn get_bucket_encryption(&self, bucket: &str) -> Request<m::GetBucketEncryptionResponse>
```

## `storage.get_bucket_lifecycle`

Get bucket lifecycle configuration

`GET /v1/buckets/{bucket}/lifecycle`

```rust
pub fn get_bucket_lifecycle(&self, bucket: &str) -> Request<m::GetBucketLifecycleResponse>
```

## `storage.get_bucket_object_lock`

Get bucket object-lock configuration

`GET /v1/buckets/{bucket}/object-lock`

```rust
pub fn get_bucket_object_lock(&self, bucket: &str) -> Request<m::GetBucketObjectLockResponse>
```

## `storage.get_bucket_policy`

Get bucket policy

`GET /v1/buckets/{bucket}/policy`

```rust
pub fn get_bucket_policy(&self, bucket: &str) -> Request<m::GetBucketPolicyResponse>
```

## `storage.get_bucket_tagging`

Get bucket tag set

`GET /v1/buckets/{bucket}/tagging`

```rust
pub fn get_bucket_tagging(&self, bucket: &str) -> Request<m::GetBucketTaggingResponse>
```

## `storage.get_bucket_versioning`

Get bucket versioning state

`GET /v1/buckets/{bucket}/versioning`

```rust
pub fn get_bucket_versioning(&self, bucket: &str) -> Request<m::GetBucketVersioningResponse>
```

## `storage.get_object`

Download object

`GET /v1/buckets/{bucket}/objects/{key}`

```rust
pub fn get_object(&self, bucket: &str, key: &str) -> BinaryRequest
```

## `storage.get_snapshot`

Get snapshot

`GET /v1/snapshots/{snapshot_id}`

```rust
pub fn get_snapshot(&self, snapshot_id: &str) -> Request<m::GetSnapshotResponse>
```

## `storage.get_snapshot_policy`

Get snapshot policy

`GET /v1/volumes/{volume_id}/snapshot-policies/{policy_id}`

```rust
pub fn get_snapshot_policy(&self, volume_id: &str, policy_id: &str) -> Request<m::GetSnapshotPolicyResponse>
```

## `storage.get_volume`

Get volume

`GET /v1/volumes/{volume_id}`

```rust
pub fn get_volume(&self, volume_id: &str) -> Request<m::GetVolumeResponse>
```

## `storage.head_bucket`

Head bucket

`HEAD /v1/buckets/{bucket}`

```rust
pub fn head_bucket(&self, bucket: &str) -> BinaryRequest
```

## `storage.head_object`

Head object

`HEAD /v1/buckets/{bucket}/objects/{key}`

```rust
pub fn head_object(&self, bucket: &str, key: &str) -> BinaryRequest
```

## `storage.initiate_multipart_upload`

Initiate a multipart upload

`POST /v1/buckets/{bucket}/multipart-uploads`

```rust
pub fn initiate_multipart_upload(&self, bucket: &str, body: &m::InitiateMultipartUploadBody) -> Request<m::InitiateMultipartUploadResponse>
```

## `storage.list_buckets`

List buckets

`GET /v1/buckets`

```rust
pub fn list_buckets(&self, query: &m::ListBucketsQuery) -> PagedRequest<m::ListBucketsResponse, m::ListBucketsItem>
```

## `storage.list_multipart_uploads`

List in-flight multipart uploads

`GET /v1/buckets/{bucket}/multipart-uploads`

```rust
pub fn list_multipart_uploads(&self, bucket: &str, query: &m::ListMultipartUploadsQuery) -> PagedRequest<m::ListMultipartUploadsResponse, m::ListMultipartUploadsItem>
```

## `storage.list_object_versions`

List object versions

`GET /v1/buckets/{bucket}/object-versions`

```rust
pub fn list_object_versions(&self, bucket: &str, query: &m::ListObjectVersionsQuery) -> PagedRequest<m::ListObjectVersionsResponse, m::ListObjectVersionsItem>
```

## `storage.list_objects`

List objects

`GET /v1/buckets/{bucket}/objects`

```rust
pub fn list_objects(&self, bucket: &str, query: &m::ListObjectsQuery) -> Request<m::ListObjectsResponse>
```

## `storage.list_parts`

List uploaded parts

`GET /v1/buckets/{bucket}/multipart-uploads/{upload_id}/parts`

```rust
pub fn list_parts(&self, bucket: &str, upload_id: &str) -> PagedRequest<m::ListPartsResponse, m::ListPartsItem>
```

## `storage.list_snapshot_policies`

List snapshot policies

`GET /v1/volumes/{volume_id}/snapshot-policies`

```rust
pub fn list_snapshot_policies(&self, volume_id: &str, query: &m::ListSnapshotPoliciesQuery) -> PagedRequest<m::ListSnapshotPoliciesResponse, m::ListSnapshotPoliciesItem>
```

## `storage.list_snapshots`

List snapshots

`GET /v1/snapshots`

```rust
pub fn list_snapshots(&self, query: &m::ListSnapshotsQuery) -> PagedRequest<m::ListSnapshotsResponse, m::ListSnapshotsItem>
```

## `storage.list_volume_types`

List volume types

`GET /v1/volume-types`

```rust
pub fn list_volume_types(&self, query: &m::ListVolumeTypesQuery) -> PagedRequest<m::ListVolumeTypesResponse, m::ListVolumeTypesItem>
```

## `storage.list_volumes`

List volumes

`GET /v1/volumes`

```rust
pub fn list_volumes(&self, query: &m::ListVolumesQuery) -> PagedRequest<m::ListVolumesResponse, m::ListVolumesItem>
```

## `storage.put_bucket_cors`

Put bucket CORS configuration

`PUT /v1/buckets/{bucket}/cors`

```rust
pub fn put_bucket_cors(&self, bucket: &str, body: &m::PutBucketCORSBody) -> EmptyRequest
```

## `storage.put_bucket_deletion_protection`

Set bucket deletion protection

`PUT /v1/buckets/{bucket}/deletion-protection`

```rust
pub fn put_bucket_deletion_protection(&self, bucket: &str, body: &m::PutBucketDeletionProtectionBody) -> EmptyRequest
```

## `storage.put_bucket_encryption`

Put bucket encryption configuration

`PUT /v1/buckets/{bucket}/encryption`

```rust
pub fn put_bucket_encryption(&self, bucket: &str, body: &m::PutBucketEncryptionBody) -> EmptyRequest
```

## `storage.put_bucket_lifecycle`

Put bucket lifecycle configuration

`PUT /v1/buckets/{bucket}/lifecycle`

```rust
pub fn put_bucket_lifecycle(&self, bucket: &str, body: &m::PutBucketLifecycleBody) -> EmptyRequest
```

## `storage.put_bucket_object_lock`

Put bucket object-lock configuration

`PUT /v1/buckets/{bucket}/object-lock`

```rust
pub fn put_bucket_object_lock(&self, bucket: &str, body: &m::PutBucketObjectLockBody) -> EmptyRequest
```

## `storage.put_bucket_policy`

Put bucket policy

`PUT /v1/buckets/{bucket}/policy`

```rust
pub fn put_bucket_policy(&self, bucket: &str, body: &m::PutBucketPolicyBody) -> EmptyRequest
```

## `storage.put_bucket_tagging`

Put bucket tag set

`PUT /v1/buckets/{bucket}/tagging`

```rust
pub fn put_bucket_tagging(&self, bucket: &str, body: &m::PutBucketTaggingBody) -> EmptyRequest
```

## `storage.put_bucket_versioning`

Set bucket versioning state

`PUT /v1/buckets/{bucket}/versioning`

```rust
pub fn put_bucket_versioning(&self, bucket: &str, body: &m::PutBucketVersioningBody) -> EmptyRequest
```

## `storage.put_object`

Upload object

`PUT /v1/buckets/{bucket}/objects/{key}`

```rust
pub fn put_object(&self, bucket: &str, key: &str, body: reqwest::Body) -> Request<m::PutObjectResponse>
```

## `storage.restore_bucket`

Restore a bucket pending deletion

`POST /v1/buckets/{bucket}/restore`

```rust
pub fn restore_bucket(&self, bucket: &str) -> EmptyRequest
```

## `storage.update_snapshot`

Update snapshot metadata

`PATCH /v1/snapshots/{snapshot_id}`

```rust
pub fn update_snapshot(&self, snapshot_id: &str, body: &m::UpdateSnapshotBody) -> Request<m::UpdateSnapshotResponse>
```

## `storage.update_snapshot_policy`

Update snapshot policy

`PATCH /v1/volumes/{volume_id}/snapshot-policies/{policy_id}`

```rust
pub fn update_snapshot_policy(&self, volume_id: &str, policy_id: &str, body: &m::UpdateSnapshotPolicyBody) -> Request<m::UpdateSnapshotPolicyResponse>
```

## `storage.update_volume`

Update volume metadata

`PATCH /v1/volumes/{volume_id}`

```rust
pub fn update_volume(&self, volume_id: &str, body: &m::UpdateVolumeBody) -> Request<m::UpdateVolumeResponse>
```

## `storage.update_volume_performance`

Update provisioned performance

`POST /v1/volumes/{volume_id}/performance`

```rust
pub fn update_volume_performance(&self, volume_id: &str, body: &m::UpdateVolumePerformanceBody) -> Request<m::UpdateVolumePerformanceResponse>
```

## `storage.upload_part`

Upload a part

`PUT /v1/buckets/{bucket}/multipart-uploads/{upload_id}/parts/{part_number}`

```rust
pub fn upload_part(&self, bucket: &str, upload_id: &str, part_number: &str, body: reqwest::Body) -> Request<m::UploadPartResponse>
```

## `telemetry.create_alert`

Create a metric alert

`POST /v1/alerts`

```rust
pub fn create_alert(&self, body: &m::CreateAlertBody) -> Request<m::CreateAlertResponse>
```

## `telemetry.create_log_group`

Create a log group

`POST /v1/log-groups`

```rust
pub fn create_log_group(&self, body: &m::CreateLogGroupBody) -> Request<m::CreateLogGroupResponse>
```

## `telemetry.delete_alert`

Delete a metric alert

`DELETE /v1/alerts/{id}`

```rust
pub fn delete_alert(&self, id: &str) -> EmptyRequest
```

## `telemetry.delete_log_group`

Delete a log group

`DELETE /v1/log-groups/{id}`

```rust
pub fn delete_log_group(&self, id: &str) -> EmptyRequest
```

## `telemetry.delete_trace_settings`

Delete trace settings

`DELETE /v1/trace-settings`

```rust
pub fn delete_trace_settings(&self) -> EmptyRequest
```

## `telemetry.get_alert`

Get a metric alert

`GET /v1/alerts/{id}`

```rust
pub fn get_alert(&self, id: &str) -> Request<m::GetAlertResponse>
```

## `telemetry.get_log`

Get a single log record by id

`GET /v1/logs/{log_id}`

```rust
pub fn get_log(&self, log_id: &str) -> Request<m::GetLogResponse>
```

## `telemetry.get_log_group`

Get a log group by id

`GET /v1/log-groups/{id}`

```rust
pub fn get_log_group(&self, id: &str) -> Request<m::GetLogGroupResponse>
```

## `telemetry.get_retained_telemetry_presence`

Check retained telemetry presence

`GET /v1/retained-data`

```rust
pub fn get_retained_telemetry_presence(&self) -> Request<m::GetRetainedTelemetryPresenceResponse>
```

## `telemetry.get_trace`

Get a trace

`GET /v1/traces/{trace_id}`

```rust
pub fn get_trace(&self, trace_id: &str) -> Request<m::GetTraceResponse>
```

## `telemetry.get_trace_settings`

Get the caller account's trace settings

`GET /v1/trace-settings`

```rust
pub fn get_trace_settings(&self) -> Request<m::GetTraceSettingsResponse>
```

## `telemetry.ingest_logs`

Ingest a batch of log records

`POST /v1/logs`

```rust
pub fn ingest_logs(&self, body: &m::IngestLogsBody) -> Request<m::IngestLogsResponse>
```

## `telemetry.ingest_spans`

Ingest a batch of trace spans

`POST /v1/spans`

```rust
pub fn ingest_spans(&self, body: &m::IngestSpansBody) -> Request<m::IngestSpansResponse>
```

## `telemetry.list_alerts`

List metric alerts

`GET /v1/alerts`

```rust
pub fn list_alerts(&self, query: &m::ListAlertsQuery) -> PagedRequest<m::ListAlertsResponse, m::ListAlertsItem>
```

## `telemetry.list_log_groups`

List log groups (or look up one by name)

`GET /v1/log-groups`

```rust
pub fn list_log_groups(&self, query: &m::ListLogGroupsQuery) -> PagedRequest<m::ListLogGroupsResponse, m::ListLogGroupsItem>
```

## `telemetry.list_metric_names`

List metric names

`GET /v1/metrics/names`

```rust
pub fn list_metric_names(&self, query: &m::ListMetricNamesQuery) -> PagedRequest<m::ListMetricNamesResponse, m::ListMetricNamesItem>
```

## `telemetry.list_metric_names_post`

List metric names (JSON)

`POST /v1/metrics/names`

```rust
pub fn list_metric_names_post(&self, body: &m::ListMetricNamesPostBody) -> Request<m::ListMetricNamesPostResponse>
```

## `telemetry.list_metric_series`

List metric series

`GET /v1/metrics/series`

```rust
pub fn list_metric_series(&self, query: &m::ListMetricSeriesQuery) -> PagedRequest<m::ListMetricSeriesResponse, m::ListMetricSeriesItem>
```

## `telemetry.list_metric_series_post`

List metric series (JSON)

`POST /v1/metrics/series`

```rust
pub fn list_metric_series_post(&self, body: &m::ListMetricSeriesPostBody) -> Request<m::ListMetricSeriesPostResponse>
```

## `telemetry.list_trace_spans`

List spans in a trace

`GET /v1/traces/{trace_id}/spans`

```rust
pub fn list_trace_spans(&self, trace_id: &str, query: &m::ListTraceSpansQuery) -> PagedRequest<m::ListTraceSpansResponse, m::ListTraceSpansItem>
```

## `telemetry.query_metrics_instant`

Run an instant metric query

`GET /v1/metrics/query`

```rust
pub fn query_metrics_instant(&self, query: &m::QueryMetricsInstantQuery) -> Request<m::QueryMetricsInstantResponse>
```

## `telemetry.query_metrics_instant_post`

Run an instant metric query (JSON)

`POST /v1/metrics/query`

```rust
pub fn query_metrics_instant_post(&self, body: &m::QueryMetricsInstantPostBody) -> Request<m::QueryMetricsInstantPostResponse>
```

## `telemetry.query_metrics_range`

Run a range metric query

`GET /v1/metrics/query-range`

```rust
pub fn query_metrics_range(&self, query: &m::QueryMetricsRangeQuery) -> Request<m::QueryMetricsRangeResponse>
```

## `telemetry.query_metrics_range_post`

Run a range metric query (JSON)

`POST /v1/metrics/query-range`

```rust
pub fn query_metrics_range_post(&self, body: &m::QueryMetricsRangePostBody) -> Request<m::QueryMetricsRangePostResponse>
```

## `telemetry.search_logs`

Search log records

`GET /v1/logs`

```rust
pub fn search_logs(&self, query: &m::SearchLogsQuery) -> Request<m::SearchLogsResponse>
```

## `telemetry.search_traces`

List traces

`GET /v1/traces`

```rust
pub fn search_traces(&self, query: &m::SearchTracesQuery) -> Request<m::SearchTracesResponse>
```

## `telemetry.update_alert`

Update a metric alert

`PATCH /v1/alerts/{id}`

```rust
pub fn update_alert(&self, id: &str, body: &m::UpdateAlertBody) -> Request<m::UpdateAlertResponse>
```

## `telemetry.update_log_group`

Update a log group

`PATCH /v1/log-groups/{id}`

```rust
pub fn update_log_group(&self, id: &str, body: &m::UpdateLogGroupBody) -> Request<m::UpdateLogGroupResponse>
```

## `telemetry.update_trace_settings`

Update the caller account's trace settings

`PATCH /v1/trace-settings`

```rust
pub fn update_trace_settings(&self, body: &m::UpdateTraceSettingsBody) -> Request<m::UpdateTraceSettingsResponse>
```

## `telemetry.write_metrics`

Prometheus remote_write ingest

`POST /v1/metrics/write`

```rust
pub fn write_metrics(&self, body: reqwest::Body) -> EmptyRequest
```

## `workspace.add_user`

Add user to organization

`POST /v1/users`

```rust
pub fn add_user(&self, body: &m::AddUserBody) -> Request<m::AddUserResponse>
```

## `workspace.add_user_to_group`

Add user to group

`POST /v1/users/{user_id}/groups`

```rust
pub fn add_user_to_group(&self, user_id: &str, body: &m::AddUserToGroupBody) -> EmptyRequest
```

## `workspace.assign_account_role`

Assign account role

`POST /v1/accounts/{account_id}/role-assignments`

```rust
pub fn assign_account_role(&self, account_id: &str, body: &m::AssignAccountRoleBody) -> Request<m::AssignAccountRoleResponse>
```

## `workspace.attach_group_policy`

Attach policy to group

`POST /v1/groups/{group_id}/policies`

```rust
pub fn attach_group_policy(&self, group_id: &str, body: &m::AttachGroupPolicyBody) -> EmptyRequest
```

## `workspace.attach_role_policy`

Attach policy to role

`POST /v1/roles/{role_id}/policies`

```rust
pub fn attach_role_policy(&self, role_id: &str, body: &m::AttachRolePolicyBody) -> EmptyRequest
```

## `workspace.attach_service_account_policy`

Attach policy to service account

`POST /v1/service-accounts/{service_account_id}/policies`

```rust
pub fn attach_service_account_policy(&self, service_account_id: &str, body: &m::AttachServiceAccountPolicyBody) -> EmptyRequest
```

## `workspace.attach_user_policy`

Attach policy to user

`POST /v1/users/{user_id}/policies`

```rust
pub fn attach_user_policy(&self, user_id: &str, body: &m::AttachUserPolicyBody) -> EmptyRequest
```

## `workspace.cancel_invitation`

Cancel invitation

`DELETE /v1/invitations/{invitation_id}`

```rust
pub fn cancel_invitation(&self, invitation_id: &str) -> EmptyRequest
```

## `workspace.create_account`

Create account

`POST /v1/accounts`

```rust
pub fn create_account(&self, body: &m::CreateAccountBody) -> Request<m::CreateAccountResponse>
```

## `workspace.create_group`

Create group

`POST /v1/groups`

```rust
pub fn create_group(&self, body: &m::CreateGroupBody) -> Request<m::CreateGroupResponse>
```

## `workspace.create_policy`

Create policy

`POST /v1/policies`

```rust
pub fn create_policy(&self, body: &m::CreatePolicyBody) -> Request<m::CreatePolicyResponse>
```

## `workspace.delete_account`

Delete account

`DELETE /v1/accounts/{account_id}`

```rust
pub fn delete_account(&self, account_id: &str) -> EmptyRequest
```

## `workspace.delete_group`

Delete group

`DELETE /v1/groups/{group_id}`

```rust
pub fn delete_group(&self, group_id: &str) -> EmptyRequest
```

## `workspace.delete_group_inline_policy`

Delete a group's inline policy by name

`DELETE /v1/groups/{group_id}/inline-policies/{policy_name}`

```rust
pub fn delete_group_inline_policy(&self, group_id: &str, policy_name: &str) -> EmptyRequest
```

## `workspace.delete_organization`

Delete organization

`DELETE /v1/organizations/{organization_id}`

```rust
pub fn delete_organization(&self, organization_id: &str) -> EmptyRequest
```

## `workspace.delete_policy`

Delete policy

`DELETE /v1/policies/{policy_id}`

```rust
pub fn delete_policy(&self, policy_id: &str) -> EmptyRequest
```

## `workspace.delete_user_inline_policy`

Delete a user's inline policy by name

`DELETE /v1/users/{user_id}/inline-policies/{policy_name}`

```rust
pub fn delete_user_inline_policy(&self, user_id: &str, policy_name: &str) -> EmptyRequest
```

## `workspace.detach_group_policy`

Detach policy from group

`DELETE /v1/groups/{group_id}/policies/{policy_id}`

```rust
pub fn detach_group_policy(&self, group_id: &str, policy_id: &str) -> EmptyRequest
```

## `workspace.detach_role_policy`

Detach policy from role

`DELETE /v1/roles/{role_id}/policies/{policy_id}`

```rust
pub fn detach_role_policy(&self, role_id: &str, policy_id: &str) -> EmptyRequest
```

## `workspace.detach_service_account_policy`

Detach policy from service account

`DELETE /v1/service-accounts/{service_account_id}/policies/{policy_id}`

```rust
pub fn detach_service_account_policy(&self, service_account_id: &str, policy_id: &str) -> EmptyRequest
```

## `workspace.detach_user_policy`

Detach policy from user

`DELETE /v1/users/{user_id}/policies/{policy_id}`

```rust
pub fn detach_user_policy(&self, user_id: &str, policy_id: &str) -> EmptyRequest
```

## `workspace.get_account`

Get account

`GET /v1/accounts/{account_id}`

```rust
pub fn get_account(&self, account_id: &str) -> Request<m::GetAccountResponse>
```

## `workspace.get_account_onboarding`

Get shared account setup progress

`GET /v1/accounts/{account_id}/onboarding`

```rust
pub fn get_account_onboarding(&self, account_id: &str) -> Request<m::GetAccountOnboardingResponse>
```

## `workspace.get_account_resources`

Check account resource presence

`GET /v1/accounts/{account_id}/resources`

```rust
pub fn get_account_resources(&self, account_id: &str) -> Request<m::GetAccountResourcesResponse>
```

## `workspace.get_group`

Get group

`GET /v1/groups/{group_id}`

```rust
pub fn get_group(&self, group_id: &str) -> Request<m::GetGroupResponse>
```

## `workspace.get_group_inline_policy`

Get a group's inline policy by name

`GET /v1/groups/{group_id}/inline-policies/{policy_name}`

```rust
pub fn get_group_inline_policy(&self, group_id: &str, policy_name: &str) -> Request<m::GetGroupInlinePolicyResponse>
```

## `workspace.get_invitation`

Get invitation

`GET /v1/invitations/{invitation_id}`

```rust
pub fn get_invitation(&self, invitation_id: &str) -> Request<m::GetInvitationResponse>
```

## `workspace.get_organization`

Get organization

`GET /v1/organizations/{organization_id}`

```rust
pub fn get_organization(&self, organization_id: &str) -> Request<m::GetOrganizationResponse>
```

## `workspace.get_policy`

Get policy

`GET /v1/policies/{policy_id}`

```rust
pub fn get_policy(&self, policy_id: &str) -> Request<m::GetPolicyResponse>
```

## `workspace.get_user`

Get user

`GET /v1/users/{user_id}`

```rust
pub fn get_user(&self, user_id: &str) -> Request<m::GetUserResponse>
```

## `workspace.get_user_inline_policy`

Get a user's inline policy by name

`GET /v1/users/{user_id}/inline-policies/{policy_name}`

```rust
pub fn get_user_inline_policy(&self, user_id: &str, policy_name: &str) -> Request<m::GetUserInlinePolicyResponse>
```

## `workspace.get_user_permission_boundary`

Get a user's permission boundary

`GET /v1/users/{user_id}/permission-boundary`

```rust
pub fn get_user_permission_boundary(&self, user_id: &str) -> Request<m::GetUserPermissionBoundaryResponse>
```

## `workspace.list_account_role_assignments`

List account role assignments

`GET /v1/accounts/{account_id}/role-assignments`

```rust
pub fn list_account_role_assignments(&self, account_id: &str) -> PagedRequest<m::ListAccountRoleAssignmentsResponse, m::ListAccountRoleAssignmentsItem>
```

## `workspace.list_account_roles`

List assigned account roles

`GET /v1/account-roles`

```rust
pub fn list_account_roles(&self) -> PagedRequest<m::ListAccountRolesResponse, m::ListAccountRolesItem>
```

## `workspace.list_accounts`

List accounts

`GET /v1/accounts`

```rust
pub fn list_accounts(&self, query: &m::ListAccountsQuery) -> PagedRequest<m::ListAccountsResponse, m::ListAccountsItem>
```

## `workspace.list_group_inline_policies`

List a group's inline policies

`GET /v1/groups/{group_id}/inline-policies`

```rust
pub fn list_group_inline_policies(&self, group_id: &str, query: &m::ListGroupInlinePoliciesQuery) -> PagedRequest<m::ListGroupInlinePoliciesResponse, m::ListGroupInlinePoliciesItem>
```

## `workspace.list_group_policies`

List group policies

`GET /v1/groups/{group_id}/policies`

```rust
pub fn list_group_policies(&self, group_id: &str, query: &m::ListGroupPoliciesQuery) -> PagedRequest<m::ListGroupPoliciesResponse, m::ListGroupPoliciesItem>
```

## `workspace.list_group_users`

List group users

`GET /v1/groups/{group_id}/users`

```rust
pub fn list_group_users(&self, group_id: &str, query: &m::ListGroupUsersQuery) -> PagedRequest<m::ListGroupUsersResponse, m::ListGroupUsersItem>
```

## `workspace.list_groups`

List groups

`GET /v1/groups`

```rust
pub fn list_groups(&self, query: &m::ListGroupsQuery) -> PagedRequest<m::ListGroupsResponse, m::ListGroupsItem>
```

## `workspace.list_invitations`

List invitations

`GET /v1/invitations`

```rust
pub fn list_invitations(&self, query: &m::ListInvitationsQuery) -> PagedRequest<m::ListInvitationsResponse, m::ListInvitationsItem>
```

## `workspace.list_organizations`

List organizations

`GET /v1/organizations`

```rust
pub fn list_organizations(&self, query: &m::ListOrganizationsQuery) -> PagedRequest<m::ListOrganizationsResponse, m::ListOrganizationsItem>
```

## `workspace.list_policies`

List policies

`GET /v1/policies`

```rust
pub fn list_policies(&self, query: &m::ListPoliciesQuery) -> PagedRequest<m::ListPoliciesResponse, m::ListPoliciesItem>
```

## `workspace.list_policy_groups`

List groups with policy

`GET /v1/policies/{policy_id}/groups`

```rust
pub fn list_policy_groups(&self, policy_id: &str, query: &m::ListPolicyGroupsQuery) -> PagedRequest<m::ListPolicyGroupsResponse, m::ListPolicyGroupsItem>
```

## `workspace.list_policy_roles`

List roles with policy

`GET /v1/policies/{policy_id}/roles`

```rust
pub fn list_policy_roles(&self, policy_id: &str, query: &m::ListPolicyRolesQuery) -> PagedRequest<m::ListPolicyRolesResponse, m::ListPolicyRolesItem>
```

## `workspace.list_policy_service_accounts`

List service accounts with policy

`GET /v1/policies/{policy_id}/service-accounts`

```rust
pub fn list_policy_service_accounts(&self, policy_id: &str, query: &m::ListPolicyServiceAccountsQuery) -> PagedRequest<m::ListPolicyServiceAccountsResponse, m::ListPolicyServiceAccountsItem>
```

## `workspace.list_policy_users`

List users with policy

`GET /v1/policies/{policy_id}/users`

```rust
pub fn list_policy_users(&self, policy_id: &str, query: &m::ListPolicyUsersQuery) -> PagedRequest<m::ListPolicyUsersResponse, m::ListPolicyUsersItem>
```

## `workspace.list_role_policies`

List role policies

`GET /v1/roles/{role_id}/policies`

```rust
pub fn list_role_policies(&self, role_id: &str, query: &m::ListRolePoliciesQuery) -> PagedRequest<m::ListRolePoliciesResponse, m::ListRolePoliciesItem>
```

## `workspace.list_service_account_policies`

List service account policies

`GET /v1/service-accounts/{service_account_id}/policies`

```rust
pub fn list_service_account_policies(&self, service_account_id: &str, query: &m::ListServiceAccountPoliciesQuery) -> PagedRequest<m::ListServiceAccountPoliciesResponse, m::ListServiceAccountPoliciesItem>
```

## `workspace.list_user_groups`

List user groups

`GET /v1/users/{user_id}/groups`

```rust
pub fn list_user_groups(&self, user_id: &str, query: &m::ListUserGroupsQuery) -> PagedRequest<m::ListUserGroupsResponse, m::ListUserGroupsItem>
```

## `workspace.list_user_inline_policies`

List a user's inline policies

`GET /v1/users/{user_id}/inline-policies`

```rust
pub fn list_user_inline_policies(&self, user_id: &str, query: &m::ListUserInlinePoliciesQuery) -> PagedRequest<m::ListUserInlinePoliciesResponse, m::ListUserInlinePoliciesItem>
```

## `workspace.list_user_policies`

List user policies

`GET /v1/users/{user_id}/policies`

```rust
pub fn list_user_policies(&self, user_id: &str, query: &m::ListUserPoliciesQuery) -> PagedRequest<m::ListUserPoliciesResponse, m::ListUserPoliciesItem>
```

## `workspace.list_users`

List users

`GET /v1/users`

```rust
pub fn list_users(&self, query: &m::ListUsersQuery) -> PagedRequest<m::ListUsersResponse, m::ListUsersItem>
```

## `workspace.put_group_inline_policy`

Create or replace a group's inline policy

`PUT /v1/groups/{group_id}/inline-policies/{policy_name}`

```rust
pub fn put_group_inline_policy(&self, group_id: &str, policy_name: &str, body: &m::PutGroupInlinePolicyBody) -> Request<m::PutGroupInlinePolicyResponse>
```

## `workspace.put_user_inline_policy`

Create or replace a user's inline policy

`PUT /v1/users/{user_id}/inline-policies/{policy_name}`

```rust
pub fn put_user_inline_policy(&self, user_id: &str, policy_name: &str, body: &m::PutUserInlinePolicyBody) -> Request<m::PutUserInlinePolicyResponse>
```

## `workspace.remove_account_role_assignment`

Remove account role assignment

`DELETE /v1/accounts/{account_id}/role-assignments/{assignment_id}`

```rust
pub fn remove_account_role_assignment(&self, account_id: &str, assignment_id: &str) -> EmptyRequest
```

## `workspace.remove_user`

Remove user from organization

`DELETE /v1/users/{user_id}`

```rust
pub fn remove_user(&self, user_id: &str) -> EmptyRequest
```

## `workspace.remove_user_from_group`

Remove user from group

`DELETE /v1/users/{user_id}/groups/{group_id}`

```rust
pub fn remove_user_from_group(&self, user_id: &str, group_id: &str) -> EmptyRequest
```

## `workspace.remove_user_permission_boundary`

Remove a user's permission boundary

`DELETE /v1/users/{user_id}/permission-boundary`

```rust
pub fn remove_user_permission_boundary(&self, user_id: &str) -> EmptyRequest
```

## `workspace.retry_account_provisioning`

Resume failed default network provisioning

`POST /v1/accounts/{account_id}/onboarding/retry`

```rust
pub fn retry_account_provisioning(&self, account_id: &str) -> Request<m::RetryAccountProvisioningResponse>
```

## `workspace.set_user_permission_boundary`

Set a user's permission boundary

`PUT /v1/users/{user_id}/permission-boundary`

```rust
pub fn set_user_permission_boundary(&self, user_id: &str, body: &m::SetUserPermissionBoundaryBody) -> EmptyRequest
```

## `workspace.update_account`

Update account

`PATCH /v1/accounts/{account_id}`

```rust
pub fn update_account(&self, account_id: &str, body: &m::UpdateAccountBody) -> Request<m::UpdateAccountResponse>
```

## `workspace.update_account_onboarding`

Update shared account guide preferences

`PATCH /v1/accounts/{account_id}/onboarding`

```rust
pub fn update_account_onboarding(&self, account_id: &str, body: &m::UpdateAccountOnboardingBody) -> Request<m::UpdateAccountOnboardingResponse>
```

## `workspace.update_group`

Update group

`PATCH /v1/groups/{group_id}`

```rust
pub fn update_group(&self, group_id: &str, body: &m::UpdateGroupBody) -> Request<m::UpdateGroupResponse>
```

## `workspace.update_organization`

Update organization

`PATCH /v1/organizations/{organization_id}`

```rust
pub fn update_organization(&self, organization_id: &str, body: &m::UpdateOrganizationBody) -> Request<m::UpdateOrganizationResponse>
```

## `workspace.update_policy`

Update policy

`PATCH /v1/policies/{policy_id}`

```rust
pub fn update_policy(&self, policy_id: &str, body: &m::UpdatePolicyBody) -> Request<m::UpdatePolicyResponse>
```
