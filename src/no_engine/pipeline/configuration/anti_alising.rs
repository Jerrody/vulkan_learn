#[derive(Default, PartialEq, Hash)]
pub enum AntiAliasingType {
    #[default]
    None,
    MSAA(u32),
    FXAA,
    TAA,
}

impl AntiAliasingType {
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    pub fn get_sample_count(&self) -> Option<ash::vk::SampleCountFlags> {
        match self {
            AntiAliasingType::MSAA(sample_count) => match sample_count {
                2 => Some(ash::vk::SampleCountFlags::TYPE_2),
                4 => Some(ash::vk::SampleCountFlags::TYPE_4),
                8 => Some(ash::vk::SampleCountFlags::TYPE_8),
                16 => Some(ash::vk::SampleCountFlags::TYPE_16),
                _ => Some(ash::vk::SampleCountFlags::TYPE_1),
            },
            _ => None,
        }
    }

    #[inline(always)]
    pub fn choose_msaa(&mut self, sample_count: u32) {
        *self = AntiAliasingType::MSAA(sample_count);
    }

    #[inline(always)]
    pub fn choose_fxaa(&mut self) {
        *self = AntiAliasingType::FXAA;
    }

    #[inline(always)]
    pub fn choose_taa(&mut self) {
        *self = AntiAliasingType::TAA;
    }
}

#[derive(Default, Hash)]
pub struct AntiAliasing {
    pub anti_aliasing_type: AntiAliasingType,
}

impl AntiAliasing {
    pub fn new(anti_alising_type: AntiAliasingType) -> Self {
        Self { anti_aliasing_type: anti_alising_type }
    }
}
