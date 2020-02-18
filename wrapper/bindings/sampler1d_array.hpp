/// @brief Include to sample 1d array textures.
/// @file gli/sampler1d_array.hpp

#pragma once

#include "sampler.hpp"
#include "texture1d_array.hpp"
#include "mipmaps_compute.hpp"
#include "convert_func.hpp"

namespace gli
{
	/// 1d array texture sampler
	/// @tparam T Sampler can fetch, write and interpret any texture format but will expose and process the data through type T conversions.
	/// @tparam P Precision in term of ULPs
	template <typename T, qualifier P = defaultp>
	class sampler1d_array : public sampler
	{
	private:
		typedef typename detail::interpolate<T>::type interpolate_type;

	public:
		typedef texture1d_array texture_type;
		typedef typename texture_type::size_type size_type;
		typedef typename texture_type::extent_type extent_type;
		typedef interpolate_type level_type;
		typedef vec<1, interpolate_type, P> normalized_type;
		typedef vec<4, T, P> texel_type;

		sampler1d_array(texture_type const& Texture, wrap Wrap, filter Mip = FILTER_NEAREST, filter Min = FILTER_NEAREST, texel_type const& BorderColor = texel_type(0, 0, 0, 1));

		~sampler1d_array() {

        }

        void set_border_color(texel_type BorderColor) {
            this->BorderColor = BorderColor;
        }

		/// Access the sampler texture object
		texture_type const& operator()() const;

		/// Fetch a texel from the sampler texture
		texel_type texel_fetch(extent_type const& TexelCoord, size_type layer, size_type Level) const;

		/// Write a texel in the sampler texture
		void texel_write(extent_type const& TexelCoord, size_type layer, size_type Level, texel_type const & Texel);

		/// Clear the sampler texture with a uniform texel
		void clear(texel_type const& Texel);

		/// Sample the sampler texture at a specific level
		texel_type texture_lod(normalized_type const& SampleCoord, size_type layer, level_type Level) const;

		/// Generate all the mipmaps of the sampler texture from the texture base level
		void generate_mipmaps(filter Minification);

		/// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included
		void generate_mipmaps(size_type BaseLayer, size_type MaxLayer, size_type BaseLevel, size_type MaxLevel, filter Minification);

	private:
		typedef typename detail::convert<texture_type, T, P>::func convert_type;
		typedef typename detail::convert<texture_type, T, P>::fetchFunc fetch_type;
		typedef typename detail::convert<texture_type, T, P>::writeFunc write_type;
		typedef typename detail::filterBase<detail::DIMENSION_1D, texture_type, interpolate_type, normalized_type, fetch_type, texel_type>::filterFunc filter_type;

		texture_type Texture;
		convert_type Convert;
		texel_type BorderColor;
		filter_type Filter;
	};

	typedef sampler1d_array<float> fsampler1DArray;
	typedef sampler1d_array<double> dsampler1DArray;
	typedef sampler1d_array<unsigned int> usampler1DArray;
	typedef sampler1d_array<int> isampler1DArray;

}//namespace gli


extern "C" {

    namespace bindings {

        namespace FSampler1DArray {

#ifndef _WIN32
            gli::fsampler1DArray fsampler1darray_new(const gli::texture1d_array & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
                return gli::fsampler1DArray(Texture, Wrap, Mip, Min);
            }
#endif

            void fsampler1darray_set_border_color(gli::fsampler1DArray & Sampler, TexelType4F BorderColor) {
                Sampler.set_border_color(gli::tex4FToVec4(BorderColor));
            }

            void fsampler1darray_clear(gli::fsampler1DArray & Sampler, TexelType4F Texel) {
                Sampler.clear(gli::tex4FToVec4(Texel));
            }

            TexelType4F fsampler1darray_texel_fetch(const gli::fsampler1DArray & Sampler, gli::fsampler1DArray::extent_type TexelCoord, gli::texture::size_type Layer, gli::texture::size_type Level) {
                gli::vec4 raw = Sampler.texel_fetch(TexelCoord, Layer, Level);
                return vec4ToTex4F(raw);
            }

            void fsampler1darray_texel_write(gli::fsampler1DArray & Sampler, gli::fsampler1DArray::extent_type TexelCoord, gli::texture::size_type Layer, gli::texture::size_type Level, TexelType4F Texel) {
                Sampler.texel_write(TexelCoord, Layer, Level, gli::tex4FToVec4(Texel));
            }

            TexelType4F fsampler1darray_texel_lod(const gli::fsampler1DArray & Sampler, float SampleCoord, gli::texture::size_type Layer, float Level) {
                gli::vec4 raw = Sampler.texture_lod(gli::fsampler1DArray::normalized_type(SampleCoord), Layer, Level);
                return vec4ToTex4F(raw);
            }

            const gli::texture1d_array & fsampler1darray_target_texture(const gli::fsampler1DArray & Sampler) {
                return Sampler.operator()();
            }

            void fsampler1darray_generate_mipmaps1(gli::fsampler1DArray & Sampler, gli::filter Minification) {
                Sampler.generate_mipmaps(Minification);
            }

            void fsampler1darray_generate_mipmaps2(gli::fsampler1DArray & Sampler, gli::texture::size_type BaseLayer, gli::texture::size_type MaxLayer, gli::texture::size_type BaseLevel, gli::texture::size_type MaxLevel, gli::filter Minification) {
                Sampler.generate_mipmaps(BaseLayer, MaxLayer, BaseLevel, MaxLevel, Minification);
            }

            /// Manually Call destructor for image object. Helper function used in FFI.
            void destroy_sampler1d_array(gli::fsampler1DArray & Sampler) {
                Sampler.~sampler1d_array();
            }
        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "sampler1d_array.inl"
#endif
