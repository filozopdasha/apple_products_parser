use anyhow::anyhow;
use apple_products_parser::*;
use pest::Parser;
use thiserror::Error;

#[test]
fn name_test() -> anyhow::Result<()> {
    ///Тестуємо валідну назву
    let pair = Grammar::parse(Rule::name, "MacBook Pro 14")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "MacBook Pro 14");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 14);

    ///Тест на неправильний вхід
    let pair = Grammar::parse(Rule::name, "&#@!");
    assert!(pair.is_err());

    ///Тест на порожній вхід
    let pair = Grammar::parse(Rule::name, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn price_test() -> anyhow::Result<()> {
    ///Тестуємо правильний формат запису для правила price
    let pair = Grammar::parse(Rule::price, "1999.99")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "1999.99");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    ///Тест на неправильний вхід(не числове значення)
    let pair = Grammar::parse(Rule::price, "19b9.99");
    assert!(pair.is_err());

    ///Тест на неправильний вхід(починається не з цифри)
    let pair = Grammar::parse(Rule::price, ".199999");
    assert!(pair.is_err());

    ///Тест на порожній вхід
    let pair = Grammar::parse(Rule::price, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn date_of_release_test() -> anyhow::Result<()> {
    ///Тестуємо правильний формат запису для правила date_of_release
    let pair = Grammar::parse(Rule::date_of_release, "24-10-2023")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "24-10-2023");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    ///Тест на неправильний вхід(є літера)
    let pair = Grammar::parse(Rule::date_of_release, "24-1o-2023i");
    assert!(pair.is_err());

    ///Тест на некоректний формат(не дотримано правила, що останнє число - 4 цифри)
    let pair = Grammar::parse(Rule::date_of_release, "24-10-20");
    assert!(pair.is_err());

    ///Тест на неправильний вхід(неправильний формат)
    let pair = Grammar::parse(Rule::date_of_release, "24-2023");
    assert!(pair.is_err());

    ///Тест на неправильний вхід(неісеуючий день)
    let pair = Grammar::parse(Rule::date_of_release, "33-10-2023");
    assert!(pair.is_err());

    ///Тест на неправильний вхід(неісеуючий місяць)
    let pair = Grammar::parse(Rule::date_of_release, "24-13-2023");
    assert!(pair.is_err());

    ///Тест на порожній вхід
    let pair = Grammar::parse(Rule::date_of_release, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn type_test() -> anyhow::Result<()> {
    /// Тестуємо правильні значення для правила type
    let pair = Grammar::parse(Rule::device_type, "Smartphone")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "Smartphone");

    let pair = Grammar::parse(Rule::device_type, "Laptop")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "Laptop");

    /// Тест на некоректне значення
    let pair = Grammar::parse(Rule::device_type, "Phone");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn screen_size_test() -> anyhow::Result<()> {
    /// Тестуємо правильний формат для правила screen_size
    let pair = Grammar::parse(Rule::screen_size, "13.3 inches")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "13.3 inches");

    /// Тест на некоректний формат (відсутній текст "inches")
    let pair = Grammar::parse(Rule::screen_size, "13.3");
    assert!(pair.is_err());

    /// Тест на некоректне значення (символ замість числа)
    let pair = Grammar::parse(Rule::screen_size, "13.a inches");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn storage_test() -> anyhow::Result<()> {
    /// Тестуємо правильний формат для правила storage
    let pair = Grammar::parse(Rule::storage, "512GB")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "512GB");

    let pair = Grammar::parse(Rule::storage, "1TB")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "1TB");

    /// Тест на некоректний формат (відсутня одиниця виміру)
    let pair = Grammar::parse(Rule::storage, "512");
    assert!(pair.is_err());

    /// Тест на некоректне значення (некоректна одиниця виміру)
    let pair = Grammar::parse(Rule::storage, "1NB");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn color_test() -> anyhow::Result<()> {
    /// Тестуємо правильний формат для правила color
    let pair = Grammar::parse(Rule::color, "Pacific Blue")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "Pacific Blue");

    /// Тест на неправильне значення (неприпустимі символи)
    let pair = Grammar::parse(Rule::color, "?!@");
    assert!(pair.is_err());

    /// Тест на порожнє значення
    let pair = Grammar::parse(Rule::color, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn availability_test() -> anyhow::Result<()> {
    /// Тестуємо правильні значення для правила availability
    let pair = Grammar::parse(Rule::availability, "true")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "true");

    let pair = Grammar::parse(Rule::availability, "false")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "false");

    /// Тест на некоректне значення
    let pair = Grammar::parse(Rule::availability, "yes");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn ram_test() -> anyhow::Result<()> {
    /// Тестуємо правильний формат для правила ram
    let pair = Grammar::parse(Rule::ram, "16GB")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "16GB");

    let pair = Grammar::parse(Rule::ram, "1TB")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "1TB");

    /// Тест на некоректний формат (відсутня одиниця виміру)
    let pair = Grammar::parse(Rule::ram, "16");
    assert!(pair.is_err());

    /// Тест на некоректне значення (некоректна одиниця виміру)
    let pair = Grammar::parse(Rule::ram, "16NB");
    assert!(pair.is_err());

    Ok(())
}
