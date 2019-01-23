use rust_onedrive::drive::endpoint::DriveEndPoint;

#[test]
fn drive_endpoint_test() {
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::Drive),
        "https://graph.microsoft.com/v1.0/drive"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveMe),
        "https://graph.microsoft.com/v1.0/me/drive"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRoot),
        "https://graph.microsoft.com/v1.0/drive/root"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootMe),
        "https://graph.microsoft.com/v1.0/me/drive/root"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootChild),
        "https://graph.microsoft.com/v1.0/drive/root/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SharedWithMe),
        "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRecent),
        "https://graph.microsoft.com/v1.0/me/drive/recent"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialDocuments),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialDocumentsChild),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialPhotos),
        "https://graph.microsoft.com/v1.0/me/drive/special/photos"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialPhotosChild),
        "https://graph.microsoft.com/v1.0/me/special/photos/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialCameraRoll),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialCameraRollChild),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialAppRoot),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialAppRootChild),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialMusic),
        "https://graph.microsoft.com/v1.0/me/drive/special/music"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialMusicChild),
        "https://graph.microsoft.com/v1.0/me/drive/special/music/children"
    );
}