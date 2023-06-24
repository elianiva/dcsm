pub struct Service {
    image: String,
    ports: Vector<(u32, u32)>,
    volumes: Vector<String>,
}
