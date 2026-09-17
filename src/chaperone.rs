use crate::openxr_data::RealOpenXrData;
use openvr as vr;
use openxr as xr;
use std::sync::Arc;

#[derive(macros::InterfaceImpl)]
#[interface = "IVRChaperone"]
#[versions(004, 003)]
pub struct Chaperone {
    vtables: Vtables,
    openxr: Arc<RealOpenXrData>,
}

impl Chaperone {
    pub fn new(openxr: Arc<RealOpenXrData>) -> Self {
        Self {
            vtables: Default::default(),
            openxr,
        }
    }
}

impl vr::IVRChaperone004_Interface for Chaperone {
    fn ResetZeroPose(&self, origin: vr::ETrackingUniverseOrigin) {
        self.openxr.reset_tracking_space(origin);
    }

    fn ForceBoundsVisible(&self, _: bool) {
        crate::warn_unimplemented!("ForceBoundsVisible");
    }
    fn AreBoundsVisible(&self) -> bool {
        crate::warn_unimplemented!("AreBoundsVisible");
        false
    }
    fn GetBoundsColor(
        &self,
        color_array: *mut vr::HmdColor_t,
        count: std::ffi::c_int,
        _collision_bounds_fade_distance: f32,
        camera_color: *mut vr::HmdColor_t,
    ) {
        crate::warn_unimplemented!("GetBoundsColor");
        if color_array.is_null() || camera_color.is_null() || count <= 0 {
            return;
        }
        let color_array = unsafe { std::slice::from_raw_parts_mut(color_array, count as usize) };
        color_array.fill(vr::HmdColor_t::default());
        unsafe {
            camera_color.write(vr::HmdColor_t::default());
        }
    }
    fn SetSceneColor(&self, _: vr::HmdColor_t) {
        crate::warn_unimplemented!("SetSceneColor");
    }
    fn ReloadInfo(&self) {
        crate::warn_unimplemented!("ReloadInfo");
    }
    fn GetPlayAreaRect(&self, rect: *mut vr::HmdQuad_t) -> bool {
        let session_data = self.openxr.session_data.get();
        let origin = match session_data.current_origin {
            vr::ETrackingUniverseOrigin::Seated => xr::ReferenceSpaceType::LOCAL,
            _ => xr::ReferenceSpaceType::STAGE,
        };
        let Ok(Some(bounds)) = session_data.session.reference_space_bounds_rect(origin) else {
            unsafe {
                *rect = Default::default();
            }
            return false;
        };

        let x = bounds.width / 2.0;
        let z = bounds.height / 2.0;
        unsafe {
            rect.write(vr::HmdQuad_t {
                vCorners: [
                    vr::HmdVector3_t {
                        v: [-x, 0.0, -z],
                    },
                    vr::HmdVector3_t { v: [-x, 0.0, z] },
                    vr::HmdVector3_t { v: [x, 0.0, z] },
                    vr::HmdVector3_t { v: [x, 0.0, -z] },
                ],
            })
        };
        true
    }
    fn GetPlayAreaSize(&self, size_x: *mut f32, size_z: *mut f32) -> bool {
        let session_data = self.openxr.session_data.get();
        let origin = match session_data.current_origin {
            vr::ETrackingUniverseOrigin::Seated => xr::ReferenceSpaceType::LOCAL,
            _ => xr::ReferenceSpaceType::STAGE,
        };
        match session_data.session.reference_space_bounds_rect(origin) {
            Ok(Some(bounds)) => {
                unsafe {
                    *size_x = bounds.width;
                    *size_z = bounds.height;
                };
                true
            }
            _ => {
                unsafe {
                    *size_x = 1.0;
                    *size_z = 1.0;
                };
                false
            }
        }
    }
    fn GetCalibrationState(&self) -> vr::ChaperoneCalibrationState {
        vr::ChaperoneCalibrationState::OK
    }
}
