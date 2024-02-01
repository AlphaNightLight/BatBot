use std::mem;
///helper
fn try_from<T: Clone>(x: &[u8]) -> Option<T> {
    let t: Vec<u8> = x.iter().take(4).copied().collect();
    if t.len() == mem::size_of::<T>() {
        let ptr = t.as_ptr();
        let ptr = ptr as *const T;
        unsafe { Some((*ptr).clone()) }
    } else {
        None
    }
}

pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

impl From<Joystick> for Vec<u8> {
    fn from(value: Joystick) -> Self {
        value
            .x
            .to_le_bytes()
            .into_iter()
            .chain(value.y.to_be_bytes().into_iter())
            .collect()
    }
}
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TryFrom<&[u8]> for Position {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 12 {
            Err("wrong size")?;
        }
        let x = try_from::<f32>(&value[0..4]).ok_or("something wrong appened")?;
        let y = try_from::<f32>(&value[4..8]).ok_or("something wrong appened")?;
        let z = try_from::<f32>(&value[8..12]).ok_or("something wrong appened")?;
        Ok(Position { x, y, z })
    }
}

pub struct CarPosition{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}
impl TryFrom<&[u8]> for CarPosition {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            Err("wrong size")?;
        }
        let x = try_from::<f32>(&value[0..4]).ok_or("something wrong appened")?;
        let y = try_from::<f32>(&value[4..8]).ok_or("something wrong appened")?;
        let z = try_from::<f32>(&value[8..12]).ok_or("something wrong appened")?;
        let angle = try_from::<f32>(&value[12..16]).ok_or("something wrong appened")?;
        Ok(CarPosition { x, y, z, angle })
    }
}