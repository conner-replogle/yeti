#[derive(Debug,Clone, Copy)]
pub struct GlowStruct
{
    pub r: f32,
	pub g: f32,
    pub b: f32,
    pub a: f32,
    pub buf1: [u8;8],
    pub unknown: f32,
    pub buf3: [u8;4],
    pub render_when_occluded: bool,
    pub render_when_unoccluded: bool,
    pub full_bloom: bool,
}
pub struct ClrRender
{
    pub red: u8,
    pub green: u8, 
    pub blue: u8
}