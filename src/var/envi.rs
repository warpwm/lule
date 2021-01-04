use crate::scheme::*;

pub fn concatinate(scheme: &mut SCHEME) {
    let env_lule_w = std::env::var("LULE_W");
    if env_lule_w.is_ok(){
        scheme.set_walldir(Some(env_lule_w.unwrap()));
    }

    let env_lule_c = std::env::var("LULE_C");
    if env_lule_c.is_ok(){
        scheme.set_config(Some(env_lule_c.unwrap()));
    }

    let env_lule_s = std::env::var("LULE_S");
    if env_lule_s.is_ok(){
        let mut newvec = vec![env_lule_s.unwrap()];
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
    if env_lule_a.is_ok(){
        scheme.set_cache(Some(env_lule_a.unwrap()));
    }
}
