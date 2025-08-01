// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag = "1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag = "1")]
    pub width: u32,
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(enumeration = "resize::ResizeType", tag = "3")]
    pub r_type: i32,
    #[prost(enumeration = "resize::SampleFilter", tag = "4")]
    pub filter: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResizeType {
        Normal = 0,
        SeamCarve = 1,
    }
    impl ResizeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Normal => "NORMAL",
                Self::SeamCarve => "SEAM_CARVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORMAL" => Some(Self::Normal),
                "SEAM_CARVE" => Some(Self::SeamCarve),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleFilter {
        Undefined = 0,
        Nearest = 1,
        Triangle = 2,
        CatmullRom = 3,
        Gaussian = 4,
        Lanczos3 = 5,
    }
    impl SampleFilter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Undefined => "UNDEFINED",
                Self::Nearest => "NEAREST",
                Self::Triangle => "TRIANGLE",
                Self::CatmullRom => "CATMULL_ROM",
                Self::Gaussian => "GAUSSIAN",
                Self::Lanczos3 => "LANCZOS3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNDEFINED" => Some(Self::Undefined),
                "NEAREST" => Some(Self::Nearest),
                "TRIANGLE" => Some(Self::Triangle),
                "CATMULL_ROM" => Some(Self::CatmullRom),
                "GAUSSIAN" => Some(Self::Gaussian),
                "LANCZOS3" => Some(Self::Lanczos3),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof = "spec::Data", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub data: ::core::option::Option<spec::Data>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Resize(super::Resize),
        #[prost(message, tag = "2")]
        Crop(super::Crop),
        #[prost(message, tag = "3")]
        Flipv(super::FlipV),
        #[prost(message, tag = "4")]
        Fliph(super::FlipH),
        #[prost(message, tag = "5")]
        Contrast(super::Contrast),
        #[prost(message, tag = "6")]
        Filter(super::Filter),
        #[prost(message, tag = "7")]
        Watermark(super::WaterMark),
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag = "1")]
    pub x1: u32,
    #[prost(uint32, tag = "2")]
    pub x2: u32,
    #[prost(uint32, tag = "3")]
    pub y1: u32,
    #[prost(uint32, tag = "4")]
    pub y2: u32,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FlipH {}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FlipV {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Contrast {
    #[prost(float, tag = "1")]
    pub contrast: f32,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Filter {
    #[prost(enumeration = "filter::Filter", tag = "1")]
    pub filter: i32,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Filter {
        Unspecified = 0,
        Oceanic = 1,
        Islands = 2,
        /// more: <https://docs.rs/photon-rs/0.3.1/photon_rs/filters/fn.filter.html>
        Marine = 3,
    }
    impl Filter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "UNSPECIFIED",
                Self::Oceanic => "OCEANIC",
                Self::Islands => "ISLANDS",
                Self::Marine => "MARINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "OCEANIC" => Some(Self::Oceanic),
                "ISLANDS" => Some(Self::Islands),
                "MARINE" => Some(Self::Marine),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WaterMark {
    #[prost(uint32, tag = "1")]
    pub x: u32,
    #[prost(uint32, tag = "2")]
    pub y: u32,
}
