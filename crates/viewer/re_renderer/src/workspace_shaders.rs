// This file is autogenerated via build.rs.
// DO NOT EDIT.

use std::path::Path;

static ONCE: ::std::sync::atomic::AtomicBool = ::std::sync::atomic::AtomicBool::new(false);

pub fn init() {
    if ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    use crate::file_system::FileSystem as _;
    let fs = crate::MemFileSystem::get();

    {
        let virtpath = Path::new("shader/colormap.wgsl");
        let content = include_str!("../shader/colormap.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/composite.wgsl");
        let content = include_str!("../shader/composite.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/conversions/yuv_converter.wgsl");
        let content = include_str!("../shader/conversions/yuv_converter.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/copy_texture.wgsl");
        let content = include_str!("../shader/copy_texture.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/debug_overlay.wgsl");
        let content = include_str!("../shader/debug_overlay.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/depth_cloud.wgsl");
        let content = include_str!("../shader/depth_cloud.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/generic_skybox.wgsl");
        let content = include_str!("../shader/generic_skybox.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/global_bindings.wgsl");
        let content = include_str!("../shader/global_bindings.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/instanced_mesh.wgsl");
        let content = include_str!("../shader/instanced_mesh.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/lines.wgsl");
        let content = include_str!("../shader/lines.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/mesh_vertex.wgsl");
        let content = include_str!("../shader/mesh_vertex.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/outlines/jumpflooding_init.wgsl");
        let content = include_str!("../shader/outlines/jumpflooding_init.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/outlines/jumpflooding_init_msaa.wgsl");
        let content = include_str!("../shader/outlines/jumpflooding_init_msaa.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/outlines/jumpflooding_init_shared.wgsl");
        let content = include_str!("../shader/outlines/jumpflooding_init_shared.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/outlines/jumpflooding_step.wgsl");
        let content = include_str!("../shader/outlines/jumpflooding_step.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/point_cloud.wgsl");
        let content = include_str!("../shader/point_cloud.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/rectangle.wgsl");
        let content = include_str!("../shader/rectangle.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/rectangle_fs.wgsl");
        let content = include_str!("../shader/rectangle_fs.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/rectangle_vs.wgsl");
        let content = include_str!("../shader/rectangle_vs.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/screen_triangle.wgsl");
        let content = include_str!("../shader/screen_triangle.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/screen_triangle_vertex.wgsl");
        let content = include_str!("../shader/screen_triangle_vertex.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/test_triangle.wgsl");
        let content = include_str!("../shader/test_triangle.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/types.wgsl");
        let content = include_str!("../shader/types.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/camera.wgsl");
        let content = include_str!("../shader/utils/camera.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/depth_offset.wgsl");
        let content = include_str!("../shader/utils/depth_offset.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/encoding.wgsl");
        let content = include_str!("../shader/utils/encoding.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/flags.wgsl");
        let content = include_str!("../shader/utils/flags.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/quaternion.wgsl");
        let content = include_str!("../shader/utils/quaternion.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/size.wgsl");
        let content = include_str!("../shader/utils/size.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/sphere_quad.wgsl");
        let content = include_str!("../shader/utils/sphere_quad.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }

    {
        let virtpath = Path::new("shader/utils/srgb.wgsl");
        let content = include_str!("../shader/utils/srgb.wgsl").into();
        fs.create_file(virtpath, content).unwrap();
    }
}
