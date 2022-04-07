// use alloc::vec;
// use alloc::vec::Vec;
// pub enum AtCommands {
//     Restart,
//     TransparentTransmission,
//     TransparentMode,
//     ServiceTimeout,
//     MultiplexConnection,
//     WifiMode,
//     JoinWifi,
//     TurnOnHotSpot,
//     EstablishConnection,
//     CreateService,
//     ShowConnectDevice,
//     ShowIp,
//     ShowWifiList,
// }
// struct CommandBuild {
//     pub at_map: Vec<(AtCommands, &'static str)>,
//     build_command: Option<Vec<&'static str>>,
// }
// impl CommandBuild {
//     pub fn new(&self, at_map: Vec<(AtCommands, &'static str)>) -> CommandBuild {
//         let AtMap: Vec<(AtCommands, &'static str)> = vec![
//             (AtCommands::Restart, "AT+RST"),
//             (AtCommands::TransparentTransmission, "AT+CIPMODE"),
//             (AtCommands::TransparentMode, "AT+CIPMODE"),
//             (AtCommands::MultiplexConnection, "AT+CIPMUX"),
//             (AtCommands::WifiMode, "AT+CWMODE"),
//             (AtCommands::JoinWifi, "AT+CWJAP"),
//             (AtCommands::EstablishConnection, "AT+CIPSTART"),
//             (AtCommands::CreateService, "AT+CIPSERVER"),
//         ];
//         CommandBuild {
//             at_map: AtMap,
//             build_command: None,
//         }
//     }
//     pub fn default(&mut self) {
//         let default_command:Vec<&'static str>=vec![
//             "AT+RST",""
//         ];

//         self.build_command
//     }
//     pub fn connect_wifi() {}
//     pub fn hot_spot() {}
//     pub fn send_data() {}
//     pub fn build(){}
// }
// pub struct Wifi<USART>
// where
//     USART: Fn() -> (),
// {
//     pub serial: stm32h7xx_hal::serial::Serial<USART>,
//     tx: stm32h7xx_hal::serial::Tx<USART>,
//     rx: stm32h7xx_hal::serial::Rx<USART>,
// }

// impl<USART> Wifi<USART>
// where
//     USART: Fn() -> (),
// {
//     fn send_command(&self) {}
//     pub fn connect_wifi(&self, wifi_name: &str, password: &str) {}
//     pub fn start_hot_spot(&self, wifi_name: &str, password: &str) {}
//     fn check_connect(&self) {}
//     pub fn send_data(&self) {}
// }
