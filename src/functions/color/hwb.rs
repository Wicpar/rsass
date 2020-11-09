use super::hsl::{percentage, to_rational2, to_rational_percent};
use super::rgb::values_from_list;
use super::{Error, SassFunction};
use crate::css::Value;
use crate::value::{Rgba, Unit};
use num_rational::Rational;
use num_traits::One;
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, hwb(hue, whiteness, blackness, alpha, channels), |s| {
        let (hue, w, b, a) = match s.get("hue")? {
            Value::List(vec, s, p) => values_from_list(&vec)
                .ok_or_else(|| badchannels(&Value::List(vec, s, p)))?,
            Value::Null => match s.get("channels")? {
                Value::List(vec, s, p) => values_from_list(&vec)
                    .ok_or_else(|| badchannels(&Value::List(vec, s, p)))?,
                v => return Err(badchannels(&v)),
            },
            hue => (
                hue,
                s.get("whiteness")?,
                s.get("blackness")?,
                s.get("alpha")?,
            ),
        };
        let hue = as_deg(&hue)?;
        let w = to_rational_percent(&w)?;
        let b = to_rational_percent(&b)?;
        let a = if a.is_null() {
            Rational::one()
        } else {
            to_rational2(&a)?
        };
        Ok(Rgba::from_hwba(hue, w, b, a).into())
    });
    def!(f, blackness(color), |args| match &args.get("color")? {
        &Value::Color(ref rgba, _) => {
            let arr = [&rgba.red, &rgba.blue, &rgba.green];
            let w = arr.iter().max().unwrap();
            Ok(percentage(Rational::one() - *w / 255))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, whiteness(color), |args| match &args.get("color")? {
        &Value::Color(ref rgba, _) => {
            let arr = [&rgba.red, &rgba.blue, &rgba.green];
            let w = arr.iter().min().unwrap();
            Ok(percentage(*w / 255))
        }
        v => Err(Error::badarg("color", v)),
    });
}

fn badchannels(v: &Value) -> Error {
    Error::badarg("Expected channels list", v)
}

fn as_deg(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(vv, u, ..) => {
            if u == &Unit::None {
                vv.as_ratio()
            } else if let Some(scale) = u.scale_to(&Unit::Deg) {
                (vv * &scale).as_ratio()
            } else {
                Err(Error::badarg("angle", &v))
            }
        }
        v => Err(Error::badarg("angle", &v)),
    }
}
