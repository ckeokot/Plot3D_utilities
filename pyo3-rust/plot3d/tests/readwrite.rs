use PLOT3D::read_plot3d;

#[cfg(test)]
mod reading_writing_tests{

    #[test]
    #[should_panic]
    fn should_read_plot3d() {
        let filename = &"../example_files/PahtCascade-ASCII.xyz";
        read_plot3d(filename, false,false);
        assert!(1==1);
        panic!("oh no");
    }
}