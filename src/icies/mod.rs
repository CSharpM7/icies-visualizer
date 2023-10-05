mod frame;

#[cfg(feature = "dev")]
#[smashline::installer]
pub fn installer() {
    install();
}

pub fn install() {
    frame::install();
}
