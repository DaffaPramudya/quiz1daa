# Quiz 1 Desain dan Analisis Algoritma Min Heap (Rust)

## Identitas
- Nama: Daffa Pramudya Ismanto
- NIM: 1313623051
- Program Studi: Ilmu Komputer 2023 A

## Fitur
- Membangun Min Heap dari array yang diberikan.
- Menampilkan setiap langkah heapification selama proses pembentukan heap.
- Mencetak heap akhir setelah semua elemen terurut sesuai aturan Min Heap.

### Langkah-langkah Kompilasi
1. Simpan kode dalam file, misalnya `quiz1.rs`.
2. Buka terminal atau command prompt dan navigasikan ke direktori tempat file disimpan.
3. Jalankan perintah berikut untuk mengompilasi:
   ```sh
   rustc quiz1.rs
   ```
4. Jalankan program dengan perintah:
   ```sh
   ./quiz1

## Contoh Output
```
Initial array: [100, 5, 9, 6, 8, 20, 10, 12, 18, 9]
Heapify step: [5, 100, 9, 6, 8, 20, 10, 12, 18, 9]
Heapify step: [5, 6, 9, 100, 8, 20, 10, 12, 18, 9]
Heapify step: [5, 6, 9, 12, 8, 20, 10, 100, 18, 9]
Final Min Heap: [5, 6, 9, 12, 8, 20, 10, 100, 18, 9]
```
