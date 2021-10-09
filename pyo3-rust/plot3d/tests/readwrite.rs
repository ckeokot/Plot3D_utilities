use plot3d::read::read_plot3d;
use std::io::Error; 

#[test]
#[should_panic]
fn should_read_plot3d() {
    let filename = &"../example_files/PahtCascade-ASCII.xyz";
    let result: Result<(),Error> = read_plot3d(filename,None,None);
    assert!(1==1);
    panic!("oh no");
}
