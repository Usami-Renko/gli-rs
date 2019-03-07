
#[cfg(test)]
mod texture {

    extern crate gli_rs as gli;

    use std::path::Path;
    use gli::texture::Texture2D;
    use gli::texture::GliTexture;

    fn print_texture_info(texture: &impl GliTexture) {

        println!("\tFaces  count: {}", texture.faces());
        println!("\tLayers count: {}", texture.layers());
        println!("\tLevels count: {}", texture.levels());
        println!("\tSize: {}", texture.size());
        println!("\tFormat: {}", texture.format());
        println!("\tTarget: {}", texture.target());
    }

    #[test]
    fn load_and_save_dds() {

        const TEST__DDS_PATH: &'static str = "./vendors/gli/data/array_r8_uint.dds";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.dds"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_dds(Path::new(TEST__DDS_PATH))
            .unwrap();

        if texture_loaded.empty() {
            assert!(true, "DDS texture is empty.");
        } else {
            let extent = texture_loaded.extent(0);

            println!("DDS Texture info:");
            println!("\tExtent: ({}, {})", extent[0], extent[1]);
            print_texture_info(&texture_loaded);
        }

        gli::save_dds(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }

    #[test]
    fn load_and_save_ktx() {

        const TEST__KTX_PATH: &'static str = "./vendors/gli/data/array_r8_uint.ktx";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.ktx"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_ktx(Path::new(TEST__KTX_PATH))
            .unwrap();

        if texture_loaded.empty() {
            assert!(true, "KTX texture is empty.");
        } else {
            let extent = texture_loaded.extent(0);

            println!("\tExtent: ({}, {})", extent[0], extent[1]);
            println!("KTX Texture info:");
            print_texture_info(&texture_loaded);
        }

        gli::save_ktx(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }

//    #[test]
//    fn shared_ptr_test() {
//
//        use crate::texture::gli::texture::inner::TextureAccessible;
//        const TEST_KTX_PATH: &'static str = "./vendors/gli/data/array_r8_uint.ktx";
//
//        let mut texture_loaded: Texture2D = gli::load_ktx(Path::new(TEST_KTX_PATH))
//            .unwrap();
//
//        unsafe {
//            let raw_texture = texture_loaded.raw_texture_mut();
//            raw_texture.is_print_shared_storage_count = true;
//            assert_eq!(raw_texture.get_shared_storage_count(), 1);
//        }
//
//        unsafe {
//            let base_level_image = texture_loaded.get_level(0);
//            let raw_texture = texture_loaded.raw_texture_mut();
//            assert_eq!(raw_texture.get_shared_storage_count(), 2);
//            assert_eq!(base_level_image.ffi.get_shared_storage_count(), 2);
//        }
//
//        unsafe {
//            let raw_texture = texture_loaded.raw_texture();
//            assert_eq!(raw_texture.get_shared_storage_count(), 1);
//        }
//    }
}
