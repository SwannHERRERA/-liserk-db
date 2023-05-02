struct Process {
    id_process: Uuid,
    status: String,
    activation_date: OffsetDateTime,
    creation_date: OffsetDateTime,
    docker_id: String,
}

pub async fn list_database(State((generator, pool)): State<(DynGenerator, PgPool)>) {
    let process = query_as!(Process, "SELECT * from PROCESS LIMIT 500");
}

pub async fn create_database(
    Path(db_name): Path<String>,
    State((generator, pool)): State<(DynGenerator, PgPool)>,
) -> Json<Value> {
    let metadatas = generate_metadata_for_database(&generator, &db_name);

    // Warnning: care port use, name used, folder used is case of error for now
    println!("{:?}", metadatas);
    let output = Command::new("docker")
        .arg("run")
        .args(metadatas)
        .output()
        .expect("command should run sucessfuly");
    // save metadata
    println!("{:?}", output);
    let lowercase_docker_id = output.stdout.to_ascii_lowercase();
    let docker_id = String::from_utf8_lossy(&lowercase_docker_id);
    let docker_id = docker_id.trim();
    let id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    let inserted_id = query!(
        "INSERT INTO PROCESS
        (id_process, status, activation_date, creation_date, docker_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id_process",
        id,
        "active",
        now,
        now,
        &docker_id
    )
    .fetch_one(&pool)
    .await;
    // .bind(output.stdout.to_ascii_lowercase())
    // .bind("active")
    // .();
    // stdout trim.... and there is the uuid
    // add default info to response like time ...
    println!("{:?}", inserted_id);
    let inserted_id = inserted_id.unwrap().id_process.to_string();
    json!({ "inserted_id": inserted_id }).into()
}

fn generate_metadata_for_database(
    generator: &DynGenerator,
    db_name: &String,
) -> [String; 14] {
    const PASSWORD_LENGTH: usize = 12;
    let username = generator.generate_username();
    let password = generator.generate_password(PASSWORD_LENGTH);
    let container_name = generator.generate_container_name();
    let foldername = generator.generate_folder_name();
    let network_port = generator.generate_port();
    let network_port = network_port.to_string();
    let password = f!("POSTGRES_PASSWORD={}", password);
    let username = f!("POSTGRES_USER={}", username);
    let data_path = f!("PGDATA=data/{}", foldername);
    let db = f!("POSTGRES_DB={}", db_name);
    [
        "-p".to_string(),
        network_port,
        "--name".to_string(),
        container_name,
        "-e".to_string(),
        password,
        "-e".to_string(),
        username,
        "-e".to_string(),
        db,
        "-e".to_string(),
        data_path,
        "-d".to_string(),
        "postgres:15".to_string(),
    ]
}
