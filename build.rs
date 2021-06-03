fn main() {
    windows::build!(
        windows::build!(Windows::Foundation::*);
        windows::build!(Windows::UI::*);
        windows::build!(Windows::Graphics::*);
    )
}
