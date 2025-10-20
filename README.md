🦈 Hello Tiburona Profesional — Clase 4 (Buen Día Builders)

Curso: Stellar + Rust — CodiGO / Buen Día Builders
Clase: #4 — Construí tu primer contrato profesional
Autora: @VianeyAlv6Alv

Última actualización: 19 de octubre de 2025

📘 Descripción

Este proyecto implementa el contrato Hello Tiburona Profesional, el primer contrato “production-ready” del curso CodiGO Futuro / Buen Día Builders.
Aplica conceptos avanzados de Rust, Soroban SDK y Stellar Futurenet, combinando:

Manejo profesional de errores (Result, Option)

Organización de storage con DataKey

Validaciones de entrada con String

Control de acceso por Address

Manejo de TTL (Time To Live)

Tests automatizados con casos de éxito y error

El objetivo es construir un contrato limpio, seguro y fácil de mantener, aplicando buenas prácticas de desarrollo en blockchain.

🛠️ Estructura del proyecto
hello-tiburona/
├─ Cargo.toml                # Workspace
├─ Soroban.toml              # Configuración de red y contratos
├─ README.md                 # Este archivo 
└─ contracts/
   └─ hello-tiburona/
      ├─ Cargo.toml          # Config del contrato
      └─ src/
         └─ lib.rs           # Código fuente + tests


⚙️ Instalación y setup
1️⃣ Requisitos previos

Asegúrate de tener instalados:

rustup target add wasm32-unknown-unknown
cargo install soroban-cli --locked



Verifica las versiones:

rustc --version
soroban --version

2️⃣ Crear identidad (solo una vez)
soroban config identity generate dev
soroban config identity address dev


Guarda la dirección que devuelve — será tu cuenta admin y firmante.

🚧 Build y pruebas locales
🧱 Compilar contrato
soroban contract build


Salida esperada:

✅ Finished release [optimized] target(s)

🧪 Ejecutar tests
cargo test


Debe mostrar:

running 6 tests
test result: ok. 6 passed; 0 failed

🚀 Despliegue en Futurenet
1️⃣ Deploy
WASM=contracts/hello-tiburona/target/wasm32-unknown-unknown/release/hello_tiburona.wasm
ID=$(soroban contract deploy --wasm $WASM --network futurenet --source dev)
echo $ID

2️⃣ Inicializar contrato
soroban contract invoke --id $ID --fn initialize --network futurenet --source dev -- \
  --admin $(soroban config identity address dev)

3️⃣ Interactuar con el contrato

Saludar:

soroban contract invoke --id $ID --fn hello --network futurenet -- \
  --usuario $(soroban config identity address dev) \
  --nombre "Ana"


Consultar contador:

soroban contract invoke --id $ID --fn get_contador --network futurenet


Consultar último saludo:

soroban contract invoke --id $ID --fn get_ultimo_saludo --network futurenet -- \
  --usuario $(soroban config identity address dev)


Resetear contador (solo admin):

soroban contract invoke --id $ID --fn reset_contador --network futurenet --source dev -- \
  --caller $(soroban config identity address dev)

🧩 Características del contrato
Módulo	Descripción
initialize()	Guarda Admin, inicializa contador y TTL
hello()	Valida String, incrementa contador, guarda saludo y extiende TTL
get_contador()	Devuelve el total de saludos globales
get_ultimo_saludo()	Retorna el último saludo de una dirección (o None)
reset_contador()	Solo el Admin puede resetear a cero
✅ Tests incluidos

Inicialización correcta

Doble inicialización (error)

Saludo exitoso con nombre válido

Error por nombre vacío

Reset solo por admin

Reset no autorizado (error)

🧠 Reflexiones de aprendizaje

String vs Symbol: String tiene .len(), .to_string(), y permite validaciones; Symbol no.

Result y Option: diferencian entre fallas controladas y valores opcionales.

Storage:

instance → variables globales (admin, contador)

persistent → datos por usuario (último saludo)

Control de acceso: verificaciones de Address o require_auth().
