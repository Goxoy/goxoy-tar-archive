# Goxoy File Archive

Dosyaları TAR dosyası içinde arşivlemeye yarayan kitaplık.

## Kullanım / Örnekler

```rust
    // arşiv adı
    let mut storage_obj = Storage::new("archive");

    // dosya ekleme işlemi 
    // dosya yoksa ekleniyor varsa güncelleniyor
    let file_added = storage_obj.add_file("dir_name/filename.extension");
    if file_added == true{
        println!("dosya eklendi");
    }else{
        println!("dosya eklenmesi esnasinda hata olustu");
    }

    // dosyayi farkli isimle ekleme işlemi
    // dosya yoksa ekleniyor varsa güncelleniyor
    let file_added = storage_obj.add_file_with_name("dir_name/filename.extension", "new_file_name.new_ext");
    if file_added == true{
        println!("dosya eklendi");
    }else{
        println!("dosya eklenmesi esnasinda hata olustu");
    }

    // dosyayi farkli isimle ekleme işlemi
    let file_removed = storage_obj.remove_file("file_name.ext");
    if file_removed == true{
        println!("dosya silindi");
    }else{
        println!("dosya silinmesi esnasinda hata olustu");
    }

    
```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)