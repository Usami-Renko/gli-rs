
use crate::ffi::root::gli;
use crate::format::{Format, Swizzle};
use crate::target::Target;
use crate::texture::{GliTexture, TextureCube};
use crate::texture::inner::TextureAccessible;
use crate::Extent2d;

/// Cube map array texture
pub struct TextureCubeArray {
    ffi: gli::texture_cube_array,
}

impl TextureCubeArray {

    /// Create an empty texture cube array.
    #[inline]
    pub fn new_empty() -> TextureCubeArray {
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new() } }
    }

    /// Create a texture_cube_array and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent2d, layers: usize, levels: usize) -> TextureCubeArray {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new1(format.0, &extent, layers, levels, &default_swizzles) } }
    }

    /// Create a texture_cube_array and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent2d, layers: usize) -> TextureCubeArray {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new2(format.0, &extent, layers, &default_swizzles) } }
    }

    /// Create a texture_cube_array view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &impl GliTexture) -> TextureCubeArray {
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new3(texture.raw_texture()) } }
    }

    /// Create a texture_cube_array view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> TextureCubeArray {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new4(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level, &default_swizzles) } }
    }

    /// Create a texture_cube_array view, reference a subset of an existing texture_cube_array instance.
    #[inline]
    pub fn new_from_subset(texture: &TextureCubeArray, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> TextureCubeArray {
        TextureCubeArray { ffi: unsafe { gli::texture_cube_array::new5(&texture.ffi, base_layer, max_layer, base_face, max_face, base_level, max_level) } }
    }

    /// Create a view of the texture identified by Layer in the texture array.
    ///
    /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_layer(&self, layer: usize) -> TextureCube {

        debug_assert!(layer < self.layers());

        TextureCube::new_detail(
            self, self.format(),
            self.base_layer() + layer, self.base_level() + layer,
            self.base_face(), self.max_face(),
            self.base_level(), self.max_level())
    }
}

impl GliTexture for TextureCubeArray {
    const TARGET_TYPE: Target = Target::TARGET_CUBE_ARRAY;
    type ExtentType = Extent2d; // equivalent to gli::texture2d_extent_type.

    /// Return the dimensions of a texture instance: width and height where both should be equal.
    fn extent(&self, level: usize) -> Self::ExtentType {
        unsafe { self.ffi.extent(level) }
    }
}

impl TextureAccessible for TextureCubeArray {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}

impl From<gli::texture> for TextureCubeArray {

    fn from(ffi: gli::texture) -> TextureCubeArray {
        TextureCubeArray { ffi: gli::texture_cube_array { _base: ffi } }
    }
}
