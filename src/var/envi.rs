use crate::scheme::*;

pub fn concatinate(scheme: &mut Scheme) {
    let env_lule_w = std::env::var("LULE_W");
    if let Ok(env_lule_w) = env_lule_w {
        scheme.set_walldir(Some(env_lule_w));
    }

    let env_lule_c = std::env::var("LULE_C");
    if let Ok(env_lule_c) = env_lule_c {
        scheme.set_config(Some(env_lule_c));
    }

    let env_lule_s = std::env::var("LULE_S");
    if let Ok(env_lule_s) = env_lule_s {
        let mut newvec = vec![env_lule_s];
        match scheme.scripts() {
            None => {
                scheme.set_scripts(Some(newvec));
            }
            Some(vec) => {
                newvec.append(&mut vec.clone());
                scheme.set_scripts(Some(newvec));
            }
        }
    }

    let env_lule_a = std::env::var("LULE_A");
    if let Ok(env_lule_a) = env_lule_a {
        scheme.set_cache(Some(env_lule_a));
    }
}
