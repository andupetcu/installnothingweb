use std::ops::Range;

#[derive(Clone)]
pub struct SimulationConfig {
    pub ai: AiConfig,
    pub cloud: CloudConfig,
    pub container: ContainerConfig,
    pub bios: BiosConfig,
    pub boot: BootConfig,
    pub bootloader: BootloaderConfig,
    pub speed_multiplier: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            cloud: CloudConfig::default(),
            container: ContainerConfig::default(),
            bios: BiosConfig::default(),
            boot: BootConfig::default(),
            bootloader: BootloaderConfig::default(),
            speed_multiplier: 1.0,
        }
    }
}

impl SimulationConfig {
    /// Create config with fast mode (50% faster delays)
    pub fn with_fast_mode(mut self) -> Self {
        self.speed_multiplier = 0.5;
        self.bios = self.bios.with_speed_multiplier(0.5);
        self.boot = self.boot.with_speed_multiplier(0.5);
        self.bootloader = self.bootloader.with_speed_multiplier(0.5);
        self
    }

    /// Create config with chaos mode (increased failure rates)
    pub fn with_chaos_mode(mut self) -> Self {
        self.ai = self.ai.with_chaos_mode();
        self.cloud = self.cloud.with_chaos_mode();
        self.container = self.container.with_chaos_mode();
        self.bios.cmos_error_chance = 0.5;
        self.bootloader.windows_found_chance = 0.8;
        self
    }
}

#[derive(Clone)]
pub struct BiosConfig {
    pub vendor: &'static str,
    pub version: &'static str,
    pub new_version: &'static str,
    pub bios_date: &'static str,
    pub header_delay: u64,
    pub post_start_delay: u64,
    pub cpu_detect_time: u64,
    pub cpu_cores_time: u64,
    pub cpu_freq_time: u64,
    pub memory_test_time: u64,
    pub memory_details_time: u64,
    pub cmos_warning_time: u64,
    pub ide_master_time: u64,
    pub ide_slave_time: u64,
    pub pci_scan_time: u64,
    pub pci_device_time: u64,
    pub network_detect_time: u64,
    pub usb_detect_time: u64,
    pub system_info_time: u64,
    pub uuid_time: u64,
    pub boot_priority_time: u64,
    pub boot_display_time: u64,
    pub firmware_header_delay: u64,
    pub backup_time: u64,
    pub verify_time: u64,
    pub warning_delay: u64,
    pub erase_min: u64,
    pub erase_max: u64,
    pub write_min: u64,
    pub write_max: u64,
    pub verify_min: u64,
    pub verify_max: u64,
    pub complete_time: u64,
    pub escd_time: u64,
    pub success_delay: u64,
    pub cmos_error_chance: f64,
}

impl Default for BiosConfig {
    fn default() -> Self {
        Self {
            vendor: "American Megatrends BIOS (C)2003-2025",
            version: "AMIBIOS v08.00.15",
            new_version: "v08.00.16",
            bios_date: "11/15/2025",
            header_delay: 400,
            post_start_delay: 400,
            cpu_detect_time: 800,
            cpu_cores_time: 600,
            cpu_freq_time: 500,
            memory_test_time: 1200,
            memory_details_time: 700,
            cmos_warning_time: 800,
            ide_master_time: 900,
            ide_slave_time: 400,
            pci_scan_time: 800,
            pci_device_time: 500,
            network_detect_time: 600,
            usb_detect_time: 500,
            system_info_time: 500,
            uuid_time: 700,
            boot_priority_time: 600,
            boot_display_time: 500,
            firmware_header_delay: 600,
            backup_time: 1800,
            verify_time: 1200,
            warning_delay: 800,
            erase_min: 1500,
            erase_max: 2500,
            write_min: 3000,
            write_max: 5000,
            verify_min: 2000,
            verify_max: 3500,
            complete_time: 800,
            escd_time: 1000,
            success_delay: 600,
            cmos_error_chance: 0.25,
        }
    }
}

impl BiosConfig {
    pub fn with_speed_multiplier(mut self, mult: f64) -> Self {
        self.header_delay = (self.header_delay as f64 * mult) as u64;
        self.post_start_delay = (self.post_start_delay as f64 * mult) as u64;
        self.cpu_detect_time = (self.cpu_detect_time as f64 * mult) as u64;
        self.cpu_cores_time = (self.cpu_cores_time as f64 * mult) as u64;
        self.cpu_freq_time = (self.cpu_freq_time as f64 * mult) as u64;
        self.memory_test_time = (self.memory_test_time as f64 * mult) as u64;
        self.memory_details_time = (self.memory_details_time as f64 * mult) as u64;
        self.cmos_warning_time = (self.cmos_warning_time as f64 * mult) as u64;
        self.ide_master_time = (self.ide_master_time as f64 * mult) as u64;
        self.ide_slave_time = (self.ide_slave_time as f64 * mult) as u64;
        self.pci_scan_time = (self.pci_scan_time as f64 * mult) as u64;
        self.pci_device_time = (self.pci_device_time as f64 * mult) as u64;
        self.network_detect_time = (self.network_detect_time as f64 * mult) as u64;
        self.usb_detect_time = (self.usb_detect_time as f64 * mult) as u64;
        self.system_info_time = (self.system_info_time as f64 * mult) as u64;
        self.uuid_time = (self.uuid_time as f64 * mult) as u64;
        self.boot_priority_time = (self.boot_priority_time as f64 * mult) as u64;
        self.boot_display_time = (self.boot_display_time as f64 * mult) as u64;
        self.firmware_header_delay = (self.firmware_header_delay as f64 * mult) as u64;
        self.backup_time = (self.backup_time as f64 * mult) as u64;
        self.verify_time = (self.verify_time as f64 * mult) as u64;
        self.warning_delay = (self.warning_delay as f64 * mult) as u64;
        self.erase_min = (self.erase_min as f64 * mult) as u64;
        self.erase_max = (self.erase_max as f64 * mult) as u64;
        self.write_min = (self.write_min as f64 * mult) as u64;
        self.write_max = (self.write_max as f64 * mult) as u64;
        self.verify_min = (self.verify_min as f64 * mult) as u64;
        self.verify_max = (self.verify_max as f64 * mult) as u64;
        self.complete_time = (self.complete_time as f64 * mult) as u64;
        self.escd_time = (self.escd_time as f64 * mult) as u64;
        self.success_delay = (self.success_delay as f64 * mult) as u64;
        self
    }
}

#[derive(Clone)]
pub struct BootConfig {
    pub log_count_range: Range<usize>,
    pub log_delay_range: Range<u64>,
    pub final_delay: u64,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            log_count_range: 8..15,
            log_delay_range: 50..200,
            final_delay: 300,
        }
    }
}

impl BootConfig {
    pub fn with_speed_multiplier(mut self, mult: f64) -> Self {
        self.log_delay_range = ((self.log_delay_range.start as f64 * mult) as u64)
            ..((self.log_delay_range.end as f64 * mult) as u64);
        self.final_delay = (self.final_delay as f64 * mult) as u64;
        self
    }
}

#[derive(Clone)]
pub struct BootloaderConfig {
    pub install_delay: u64,
    pub probe_delay: u64,
    pub device_install_delay: u64,
    pub config_gen_delay: u64,
    pub kernel_scan_delay_range: Range<u64>,
    pub windows_found_chance: f64,
    pub windows_delay: u64,
    pub write_stage_delay_range: Range<u64>,
    pub finish_delay: u64,
}

impl Default for BootloaderConfig {
    fn default() -> Self {
        Self {
            install_delay: 800,
            probe_delay: 600,
            device_install_delay: 500,
            config_gen_delay: 700,
            kernel_scan_delay_range: 200..400,
            windows_found_chance: 0.3,
            windows_delay: 400,
            write_stage_delay_range: 400..800,
            finish_delay: 500,
        }
    }
}

impl BootloaderConfig {
    pub fn with_speed_multiplier(mut self, mult: f64) -> Self {
        self.install_delay = (self.install_delay as f64 * mult) as u64;
        self.probe_delay = (self.probe_delay as f64 * mult) as u64;
        self.device_install_delay = (self.device_install_delay as f64 * mult) as u64;
        self.config_gen_delay = (self.config_gen_delay as f64 * mult) as u64;
        self.kernel_scan_delay_range = ((self.kernel_scan_delay_range.start as f64 * mult) as u64)
            ..((self.kernel_scan_delay_range.end as f64 * mult) as u64);
        self.windows_delay = (self.windows_delay as f64 * mult) as u64;
        self.write_stage_delay_range = ((self.write_stage_delay_range.start as f64 * mult) as u64)
            ..((self.write_stage_delay_range.end as f64 * mult) as u64);
        self.finish_delay = (self.finish_delay as f64 * mult) as u64;
        self
    }
}

#[derive(Clone)]
pub struct AiConfig {
    pub model_download_speed_range: Range<u64>,
    pub failure_rate_network: f64,
    pub failure_rate_checksum: f64,
    pub failure_rate_kernel_panic: f64,
    pub failure_rate_oom: f64,
    pub layer_load_delay_range: Range<u64>,
    pub compilation_speed_range: Range<u64>,
    pub checksum_delay_range: Range<u64>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model_download_speed_range: 1000..3000,
            failure_rate_network: 0.15,
            failure_rate_checksum: 0.05,
            failure_rate_kernel_panic: 0.1,
            failure_rate_oom: 0.2,
            layer_load_delay_range: 300..800,
            compilation_speed_range: 800..2000,
            checksum_delay_range: 500..1500,
        }
    }
}

impl AiConfig {
    pub fn with_chaos_mode(mut self) -> Self {
        self.failure_rate_network = 0.4;
        self.failure_rate_checksum = 0.2;
        self.failure_rate_kernel_panic = 0.3;
        self.failure_rate_oom = 0.5;
        self
    }
}

#[derive(Clone)]
pub struct CloudConfig {
    pub failure_rate_rate_limit: f64,
    pub failure_rate_insufficient_capacity: f64,
    pub failure_rate_dependency_violation: f64,
    pub failure_rate_checksum_mismatch: f64,
    pub provision_speed_range: Range<u64>,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            failure_rate_rate_limit: 0.05,
            failure_rate_insufficient_capacity: 0.3,
            failure_rate_dependency_violation: 0.3,
            failure_rate_checksum_mismatch: 0.2,
            provision_speed_range: 300..1200,
        }
    }
}

impl CloudConfig {
    pub fn with_chaos_mode(mut self) -> Self {
        self.failure_rate_rate_limit = 0.2;
        self.failure_rate_insufficient_capacity = 0.6;
        self.failure_rate_dependency_violation = 0.5;
        self.failure_rate_checksum_mismatch = 0.4;
        self
    }
}

#[derive(Clone)]
pub struct ContainerConfig {
    pub failure_rate_image_pull: f64,
    pub failure_rate_readiness_probe: f64,
    pub failure_rate_crash_loop: f64,
    pub probability_volume_mount: f64,
    pub probability_secret_mount: f64,
    pub probability_sidecar_injection: f64,
    pub layer_pull_speed_range: Range<u64>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            failure_rate_image_pull: 0.15,
            failure_rate_readiness_probe: 0.2,
            failure_rate_crash_loop: 0.05,
            probability_volume_mount: 0.4,
            probability_secret_mount: 0.3,
            probability_sidecar_injection: 0.6,
            layer_pull_speed_range: 150..2500,
        }
    }
}

impl ContainerConfig {
    pub fn with_chaos_mode(mut self) -> Self {
        self.failure_rate_image_pull = 0.4;
        self.failure_rate_readiness_probe = 0.5;
        self.failure_rate_crash_loop = 0.3;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SimulationConfig::default();
        assert_eq!(config.speed_multiplier, 1.0);
        assert_eq!(config.bios.cmos_error_chance, 0.25);
        assert_eq!(config.bootloader.windows_found_chance, 0.3);
    }

    #[test]
    fn test_fast_mode() {
        let config = SimulationConfig::default().with_fast_mode();
        assert_eq!(config.speed_multiplier, 0.5);
        // Check that delays are halved
        assert_eq!(config.bios.header_delay, 200); // 400 * 0.5
        assert_eq!(config.boot.final_delay, 150);  // 300 * 0.5
        assert_eq!(config.bootloader.install_delay, 400); // 800 * 0.5
    }

    #[test]
    fn test_chaos_mode() {
        let config = SimulationConfig::default().with_chaos_mode();
        // Check increased failure rates
        assert_eq!(config.bios.cmos_error_chance, 0.5);
        assert_eq!(config.bootloader.windows_found_chance, 0.8);
        assert_eq!(config.ai.failure_rate_network, 0.4);
        assert_eq!(config.cloud.failure_rate_insufficient_capacity, 0.6);
        assert_eq!(config.container.failure_rate_crash_loop, 0.3);
    }

    #[test]
    fn test_fast_and_chaos_combined() {
        let config = SimulationConfig::default()
            .with_fast_mode()
            .with_chaos_mode();
        // Should have both fast delays and high failure rates
        assert_eq!(config.speed_multiplier, 0.5);
        assert_eq!(config.bios.header_delay, 200);
        assert_eq!(config.ai.failure_rate_oom, 0.5);
    }

    #[test]
    fn test_bios_config_speed_multiplier() {
        let config = BiosConfig::default().with_speed_multiplier(0.25);
        assert_eq!(config.header_delay, 100); // 400 * 0.25
        assert_eq!(config.memory_test_time, 300); // 1200 * 0.25
    }

    #[test]
    fn test_boot_config_speed_multiplier() {
        let config = BootConfig::default().with_speed_multiplier(2.0);
        assert_eq!(config.final_delay, 600); // 300 * 2.0
        assert_eq!(config.log_delay_range.start, 100); // 50 * 2.0
        assert_eq!(config.log_delay_range.end, 400); // 200 * 2.0
    }

    #[test]
    fn test_ai_chaos_mode() {
        let config = AiConfig::default().with_chaos_mode();
        assert!(config.failure_rate_network > 0.3);
        assert!(config.failure_rate_oom > 0.4);
    }

    #[test]
    fn test_cloud_chaos_mode() {
        let config = CloudConfig::default().with_chaos_mode();
        assert!(config.failure_rate_rate_limit > 0.1);
        assert!(config.failure_rate_insufficient_capacity > 0.5);
    }

    #[test]
    fn test_container_chaos_mode() {
        let config = ContainerConfig::default().with_chaos_mode();
        assert!(config.failure_rate_image_pull > 0.3);
        assert!(config.failure_rate_crash_loop > 0.2);
    }
}
