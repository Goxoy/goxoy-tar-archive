[![Version](https://img.shields.io/crates/v/goxoy-tar-archive)](https://crates.io/crates/goxoy-tar-archive)
[![Downloads](https://img.shields.io/crates/d/goxoy-tar-archive)](https://crates.io/crates/goxoy-tar-archive)
[![License](https://img.shields.io/crates/l/goxoy-file-chunker)](https://crates.io/crates/goxoy-tar-archive)
[![Docs](https://docs.rs/goxoy-tar-archive/badge.svg)](https://docs.rs/goxoy-tar-archive)

# Goxoy File Archive

Dosyaları TAR dosyası içinde arşivlemeye yarayan kitaplık.

## Kullanım / Örnekler

```rust
    // arşiv adı
    let mut tar_obj = tar_archive::new("archive");

    // dosya ekleme işlemi 
    // dosya yoksa ekleniyor varsa güncelleniyor
    let file_added : bool = tar_obj.add_file("dir_name/filename.extension");
    if file_added == true{
        println!("dosya eklendi");
    }else{
        println!("dosya eklenmesi esnasinda hata olustu");
    }

    // dosyayi farkli isimle ekleme işlemi
    // dosya yoksa ekleniyor varsa güncelleniyor
    let file_added : bool = tar_obj.add_file_with_name("dir_name/filename.extension", "new_file_name.new_ext");
    if file_added == true{
        println!("dosya eklendi");
    }else{
        println!("dosya eklenmesi esnasinda hata olustu");
    }

    // dosyayi farkli isimle ekleme işlemi
    let file_removed : bool = tar_obj.remove_file("file_name.ext");
    if file_removed == true{
        println!("dosya silindi");
    }else{
        println!("dosya silinmesi esnasinda hata olustu");
    }

    // arşivden dosyayı oku
    let file_data:Option<Vec<u8>> = tar_obj.extract_as_vec("file_name.ext");
    if file_data.is_some(){
        let raw_data = file_data.unwrap();
    }else{
        println!("dosya bulunamadi");
    }

    // arşivdeki dosyayı kaydet
    let file_saved:bool = tar_obj.save_to_file("file_name.ext","dir_name/save_file_name.ext");
    if file_saved == true {
        println!("dosya kaydedildi");
    }else{
        println!("dosya kayit islemi hatali");
    }

    
```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)