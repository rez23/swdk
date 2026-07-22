#[derive(Debug)]
pub enum HidDeviceUsageResponse {
    Pointer = 0x01,
    Mouse = 0x02,
    Joystick = 0x04,
    Gamepad = 0x05,
    Keyboard = 0x06,
    MultiaxisController = 0x07,
}

#[repr(u16)]
#[derive(Debug)]
pub enum GamepadProducers {
    Microsoft = 0x045E,
    Sony = 0x054C,
}

#[repr(u16)]
#[derive(Debug)]
pub enum MsGamepadModels {
    Xbox360 = 0x028E,
    XboxOne = 0x0B00,
    XboxElite = 0x02FF,
    //Unknown = 0x0000,
}

#[repr(u16)]
#[derive(Debug)]
pub enum SonyGamepadModels {
    DualShock3 = 0x02DD,
    DualShock4 = 0x05C4,
    DualSense = 0x0CE6, // PS5
    //Unknown = 0x0000,
}

#[derive(Debug)]
pub enum GamepadModels {
    XboxController(MsGamepadModels),
    PlaystationController(SonyGamepadModels),
    Unknown((u16, u16)),
}

impl Default for GamepadModels {
    fn default() -> Self {
        Self::Unknown((0, 0))
    }
}

impl core::fmt::Display for HidDeviceUsageResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Gamepad => write!(f, "Gamepad"),
            Self::Joystick => write!(f, "Joystick"),
            Self::Keyboard => write!(f, "Keyboard"),
            Self::Pointer => write!(f, "Pointer"),
            Self::MultiaxisController => write!(f, "Multiaxis Controller"),
            Self::Mouse => write!(f, "Mouse"),
        }
    }
}

#[allow(dead_code)]
impl HidDeviceUsageResponse {
    pub fn from_u16(u: u16) -> Option<Self> {
        match u {
            0x01 => Some(Self::Gamepad),
            0x02 => Some(Self::Joystick),
            0x06 => Some(Self::Keyboard),
            0x04 => Some(Self::Pointer),
            0x05 => Some(Self::MultiaxisController),
            0x03 => Some(Self::Mouse),
            _ => None,
        }
    }
}

impl MsGamepadModels {
    pub fn from_pid(pid: u16) -> Option<Self> {
        const XBOX360: u16 = MsGamepadModels::Xbox360 as u16;
        const XBOXONE: u16 = MsGamepadModels::XboxOne as u16;
        const XBOXELITE: u16 = MsGamepadModels::XboxElite as u16;

        match pid {
            XBOX360 => Some(Self::Xbox360),
            XBOXONE => Some(Self::XboxOne),
            XBOXELITE => Some(Self::XboxElite),
            _ => None,
        }
    }
}
impl core::fmt::Display for MsGamepadModels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Xbox360 => write!(f, "Xbox 360"),
            Self::XboxOne => write!(f, "Xbox One"),
            Self::XboxElite => write!(f, "Xbox Elite"),
        }
    }
}

impl SonyGamepadModels {
    pub fn from_pid(pid: u16) -> Option<Self> {
        const DUALSHOCK3: u16 = SonyGamepadModels::DualShock3 as u16;
        const DUALSHOCK4: u16 = SonyGamepadModels::DualShock4 as u16;
        const DUALSENSE: u16 = SonyGamepadModels::DualSense as u16;

        match pid {
            DUALSHOCK3 => Some(Self::DualShock3),
            DUALSHOCK4 => Some(Self::DualShock4),
            DUALSENSE => Some(Self::DualSense),
            _ => None,
        }
    }
}
impl core::fmt::Display for SonyGamepadModels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DualShock3 => write!(f, "DualShock 3"),
            Self::DualShock4 => write!(f, "DualShock 4"),
            Self::DualSense => write!(f, "DualSense"),
        }
    }
}

impl GamepadModels {
    pub fn from_vid_and_pid(vid: u16, pid: u16) -> Self {
        const MICROSOFT: u16 = GamepadProducers::Microsoft as u16;
        const SONY: u16 = GamepadProducers::Sony as u16;

        match vid {
            MICROSOFT => {
                MsGamepadModels::from_pid(pid)
                    .map_or(
                        Self::Unknown((vid, pid)),
                        Self::XboxController,
                    )
            },
            SONY => {
                SonyGamepadModels::from_pid(pid)
                    .map_or(
                        Self::Unknown((vid, pid)),
                        Self::PlaystationController,
                    )
            },
            _ => Self::Unknown((vid, pid)),
        }

    }
}
impl core::fmt::Display for GamepadModels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::XboxController(model) => write!(f, "Xbox Controller ({model:?})"),
            Self::PlaystationController(model) => write!(f, "PlayStation Controller ({model:?})"),
            Self::Unknown((vid, pid)) => write!(f, "Unknown Gamepad device (VID: {vid:#06X}, PID: {pid:#06X})"),
        }
    }
}

/// Returns true if the given VID/PID pair corresponds to a supported gamepad.
pub fn is_supported_gamepad(vid: u16, pid: u16) -> bool {
    // Microsoft
    if vid == GamepadProducers::Microsoft as u16 {
        return MsGamepadModels::from_pid(pid).is_some();
    }

    // Sony
    if vid == GamepadProducers::Sony as u16 {
        return SonyGamepadModels::from_pid(pid).is_some();
    }

    false
}
