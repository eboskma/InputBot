use ::*;
use KeybdKey::*;
use MouseButton::*;

impl From<KeybdKey> for u64 {
    fn from(key: KeybdKey) -> u64 {
        match key {
            BackspaceKey => 0xFF08,
            TabKey => 0xFF09,
            EnterKey => 0xFF8D,
            EscapeKey => 0xFF1B,
            SpaceKey => 0x020,
            HomeKey => 0xFF50,
            LeftKey => 0xFF51,
            UpKey => 0xFF52,
            RightKey => 0xFF53,
            DownKey => 0xFF54,
            InsertKey => 0xFF63,
            DeleteKey => 0xFF9F,
            Numrow0Key => 0x030,
            Numrow1Key => 0x031,
            Numrow2Key => 0x032,
            Numrow3Key => 0x033,
            Numrow4Key => 0x034,
            Numrow5Key => 0x035,
            Numrow6Key => 0x036,
            Numrow7Key => 0x037,
            Numrow8Key => 0x038,
            Numrow9Key => 0x039,
            AKey => 0x041,
            BKey => 0x042,
            CKey => 0x043,
            DKey => 0x044,
            EKey => 0x045,
            FKey => 0x046,
            GKey => 0x047,
            HKey => 0x048,
            IKey => 0x049,
            JKey => 0x04A,
            KKey => 0x04B,
            LKey => 0x04C,
            MKey => 0x04D,
            NKey => 0x04E,
            OKey => 0x04F,
            PKey => 0x050,
            QKey => 0x051,
            RKey => 0x052,
            SKey => 0x053,
            TKey => 0x054,
            UKey => 0x055,
            VKey => 0x056,
            WKey => 0x057,
            XKey => 0x058,
            YKey => 0x059,
            ZKey => 0x05A,
            Numpad0Key => 0xFFB0,
            Numpad1Key => 0xFFB1,
            Numpad2Key => 0xFFB2,
            Numpad3Key => 0xFFB3,
            Numpad4Key => 0xFFB4,
            Numpad5Key => 0xFFB5,
            Numpad6Key => 0xFFB6,
            Numpad7Key => 0xFFB7,
            Numpad8Key => 0xFFB8,
            Numpad9Key => 0xFFB9,
            F1Key => 0xFFBE,
            F2Key => 0xFFBF,
            F3Key => 0xFFC0,
            F4Key => 0xFFC1,
            F5Key => 0xFFC2,
            F6Key => 0xFFC3,
            F7Key => 0xFFC4,
            F8Key => 0xFFC5,
            F9Key => 0xFFC6,
            F10Key => 0xFFC7,
            F11Key => 0xFFC8,
            F12Key => 0xFFC9,
            F13Key => 0xFFCA,
            F14Key => 0xFFCB,
            F15Key => 0xFFCC,
            F16Key => 0xFFCD,
            F17Key => 0xFFCE,
            F18Key => 0xFFCF,
            F19Key => 0xFFD0,
            F20Key => 0xFFD1,
            F21Key => 0xFFD2,
            F22Key => 0xFFD3,
            F23Key => 0xFFD4,
            F24Key => 0xFFD5,
            NumLockKey => 0xFF7F,
            ScrollLockKey => 0xFF14,
            CapsLockKey => 0xFFE5,
            LShiftKey => 0xFFE1,
            RShiftKey => 0xFFE2,
            LControlKey => 0xFFE3,
            RControlKey => 0xFFE4,
            OtherKey(keycode) => keycode,
        }
    }
}

impl From<u32> for MouseButton {
    fn from(keycode: u32) -> MouseButton {
        match keycode {
            1 => LeftButton,
            2 => MiddleButton,
            3 => RightButton,
            4 => X1Button,
            5 => X2Button,
            _ => OtherButton(keycode),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            LeftButton => 1,
            MiddleButton => 2,
            RightButton => 3,
            X1Button => 4,
            X2Button => 5,
            OtherButton(keycode) => keycode,
        }
    }
}


// https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
pub fn scan_code_to_key(scan_code: u32) -> Option<KeybdKey> {
    match scan_code {
        0x0e => Some(BackspaceKey),
        0x0f => Some(TabKey),
        0x1c => Some(EnterKey),
        0x01 => Some(EscapeKey),
        0x39 => Some(SpaceKey),
        0x47 => Some(HomeKey),
        0x4b => Some(LeftKey),
        0x48 => Some(UpKey),
        0x4d => Some(RightKey),
        0x50 => Some(DownKey),
        0x52 => Some(InsertKey),
        0x53 => Some(DeleteKey),
        0x0b => Some(Numrow0Key),
        0x02 => Some(Numrow1Key),
        0x03 => Some(Numrow2Key),
        0x04 => Some(Numrow3Key),
        0x05 => Some(Numrow4Key),
        0x06 => Some(Numrow5Key),
        0x07 => Some(Numrow6Key),
        0x08 => Some(Numrow7Key),
        0x09 => Some(Numrow8Key),
        0x0a => Some(Numrow9Key),
        0x1e => Some(AKey),
        0x30 => Some(BKey),
        0x2e => Some(CKey),
        0x20 => Some(DKey),
        0x12 => Some(EKey),
        0x21 => Some(FKey),
        0x22 => Some(GKey),
        0x23 => Some(HKey),
        0x17 => Some(IKey),
        0x24 => Some(JKey),
        0x25 => Some(KKey),
        0x26 => Some(LKey),
        0x32 => Some(MKey),
        0x31 => Some(NKey),
        0x18 => Some(OKey),
        0x19 => Some(PKey),
        0x10 => Some(QKey),
        0x13 => Some(RKey),
        0x1f => Some(SKey),
        0x14 => Some(TKey),
        0x16 => Some(UKey),
        0x2f => Some(VKey),
        0x11 => Some(WKey),
        0x2d => Some(XKey),
        0x15 => Some(YKey),
        0x2c => Some(ZKey),
        0x52 => Some(Numpad0Key),
        0x4f => Some(Numpad1Key),
        0x50 => Some(Numpad2Key),
        0x51 => Some(Numpad3Key),
        0x4b => Some(Numpad4Key),
        0x4c => Some(Numpad5Key),
        0x4d => Some(Numpad6Key),
        0x47 => Some(Numpad7Key),
        0x48 => Some(Numpad8Key),
        0x49 => Some(Numpad9Key),
        0x3b => Some(F1Key),
        0x3c => Some(F2Key),
        0x3d => Some(F3Key),
        0x3e => Some(F4Key),
        0x3f => Some(F5Key),
        0x40 => Some(F6Key),
        0x41 => Some(F7Key),
        0x42 => Some(F8Key),
        0x43 => Some(F9Key),
        0x44 => Some(F10Key),
        0x45 => Some(NumLockKey),
        0x46 => Some(ScrollLockKey),
        0x3a => Some(CapsLockKey),
        0x2a => Some(LShiftKey),
        0x36 => Some(RShiftKey),
        0x1d => Some(LControlKey),
        _ => None,
    }
}
