# rja

![Hrdja Logo](https://github.com/njelich/hrdja/assets/12912633/20c147e4-e32b-4958-86e1-d435423c5fa7)


This is not finished and it will not work.


Aren't you _utrujen_ from writing Rust programs in English? Do you like saying
"razlaz" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Slovenian touch to your
programs?

**rja** (Slovenian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Slovenian, using Slovenian keywords, Slovenian function names,
Slovenian idioms.

This has been designed to be used as the official programming language to
develop the future Slovenian sovereign software.

Don't worry!
Slovenian Rust is fully compatible with English Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with rja:

### trait and impl (aka lastnost in implementacija)

```rust
hrđa::hrđa! {
    uporablja std::collections::HashMap kot Slovar;

    lastnost KljučVrednost {
        fn zapiši(&se, ključ: NizZnakov, vrednost: NizZnakov);
        fn pridobi(&se, ključ: NizZnakov) -> Rezultat<Možnost<&NizZnakov>, NizZnakov>;
    }

    nepokretno spremenljivo SLOVAR: Možnost<Slovar<NizZnakov, NizZnakov>> = Nič;

    struktura GPKrk;

    implementacija KljučVrednost za GPKrk {
        fn zapiši(&se, ključ: NizZnakov, vrednost: NizZnakov) {
            dopusti slovar = nevarno {
                SLOVAR.pridobi_ali_vstavi_z(Podrazumevano::podrazumevano)
            };
            slovar.vstavi(ključ, vrednost);
        }
        fn pridobi(&se, ključ: NizZnakov) -> Rezultat<Možnost<&NizZnakov>, NizZnakov> {
            če dopusti Nekaj(slovar) = nevarno { SLOVAR.ko_upuć() } {
                Vredu(slovar.pridobi(&ključ))
            } drugače {
                Napaka("pridobi slovar".pretvori())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[dopusti(izvor_nedostupan)]
fn sekundarni() {
    panika!("o ne"); // for the usual Croatian experience
    razlaz!("gasi to"); // a student party broken up
    racija!("nemoj, Milane"); // time to give a cut of the rakija
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Slovenian [rja](https://github.com/CppHacker-dev/rja)
