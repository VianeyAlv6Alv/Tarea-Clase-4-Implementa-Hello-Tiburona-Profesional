# 🦈 Hello Tiburona Profesional — Clase 4

![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?logo=rust&logoColor=white)
![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-22-blue?logo=stellar&logoColor=white)
![Status](https://img.shields.io/badge/Build-Passing-brightgreen?logo=githubactions&logoColor=white)
![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-yellow)
![Buen Día Builders](https://img.shields.io/badge/Buen%20Día%20Builders-%F0%9F%8C%9F-lightgrey)

> **Curso:** Stellar + Rust — CodiGO / Buen Día Builders  
> **Clase 4:** *Implementa Hello Tiburona Profesional*  
> **Autora:** [@VianeyAlv6Alv](https://github.com/VianeyAlv6Alv)  
> **Última actualización:** 19 de octubre de 2025  

---

## 🧭 Descripción

Este contrato implementa **Hello Tiburona Profesional**, el primer *smart contract* profesional del curso **CodiGO Futuro / Buen Día Builders**.  
Aplica los fundamentos de **Rust**, **Soroban SDK** y **Stellar Futurenet**, con un enfoque en **buenas prácticas**, **validaciones** y **testing profesional**.

> 📚 *“Esto no es un Hello World, es tu primer contrato production-ready.”*

---

## 🧩 Características principales

| Módulo | Descripción |
|--------|--------------|
| 🛠️ **initialize()** | Guarda `Admin`, inicializa contador y TTL |
| 💬 **hello()** | Valida nombre (`String`), incrementa contador y guarda saludo |
| 🔍 **get_contador()** | Devuelve el total de saludos globales |
| 👋 **get_ultimo_saludo()** | Retorna el último saludo de una dirección |
| 🔐 **reset_contador()** | Solo el `Admin` puede resetear el contador |

---

## 🧱 Estructura del proyecto

hello-tiburona/
├─ Cargo.toml # Workspace
├─ Soroban.toml # Configuración de red y contratos
├─ README.md # Este archivo ✨
└─ contracts/
└─ hello-tiburona/
├─ Cargo.toml # Config del contrato
└─ src/
└─ lib.rs # Código fuente + tests

---

## ⚙️ Instalación y setup

### 🔧 Requisitos previos

Asegúrate de tener instalados:

```bash
rustup target add wasm32-unknown-unknown
cargo install soroban-cli --locked
Verifica las versiones:

bash
Copiar código
rustc --version
soroban --version
🪪 Crear identidad (solo una vez)
bash
Copiar código
soroban config identity generate dev
soroban config identity address dev
Guarda la dirección que devuelve: será tu cuenta admin y firmante.

🧪 Compilación y tests
🧱 Build del contrato
bash
Copiar código
soroban contract build
Salida esperada:

scss
Copiar código
✅ Finished release [optimized] target(s)
🧪 Ejecutar tests
bash
Copiar código
cargo test
Resultado:

sql
Copiar código
running 6 tests
test result: ok. 6 passed; 0 failed
🚀 Despliegue en Futurenet
1️⃣ Deploy del contrato
bash
Copiar código
WASM=contracts/hello-tiburona/target/wasm32-unknown-unknown/release/hello_tiburona.wasm
ID=$(soroban contract deploy --wasm $WASM --network futurenet --source dev)
echo $ID
2️⃣ Inicializar
bash
Copiar código
soroban contract invoke --id $ID --fn initialize --network futurenet --source dev -- \
  --admin $(soroban config identity address dev)
3️⃣ Probar funciones
👉 Saludar

bash
Copiar código
soroban contract invoke --id $ID --fn hello --network futurenet -- \
  --usuario $(soroban config identity address dev) \
  --nombre "Ana"
📊 Consultar contador

bash
Copiar código
soroban contract invoke --id $ID --fn get_contador --network futurenet
📜 Ver último saludo

bash
Copiar código
soroban contract invoke --id $ID --fn get_ultimo_saludo --network futurenet -- \
  --usuario $(soroban config identity address dev)
🔄 Resetear contador (solo admin)

bash
Copiar código
soroban contract invoke --id $ID --fn reset_contador --network futurenet --source dev -- \
  --caller $(soroban config identity address dev)

