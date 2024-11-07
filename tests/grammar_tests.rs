use pest::Parser;
use anyhow::anyhow;
use apple_products_parser::*;


#[test]
fn name_test() -> anyhow::Result< () >
{
///Тестуємо валідну назву
    let pair = Grammar::parse(Rule::name, "MacBook Pro 14")?.next().ok_or_else(
        ||anyhow!("no pair")
    )?;

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
fn price_test() -> anyhow::Result< () >
{
    ///Тестуємо правильний формат запису для правила price
    let pair = Grammar::parse(Rule::price, "1999.99")?
    .next()
    .ok_or_else(||anyhow!("no pair"))?;

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
fn date_of_release_test() -> anyhow::Result< () >
{
    ///Тестуємо правильний формат запису для правила date_of_release
    let pair = Grammar::parse(Rule::date_of_release, "24-10-2023")?
    .next()
    .ok_or_else(||anyhow!("no pair"))?;

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
