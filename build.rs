extern crate cc;

fn main(){
    cc::Build::new()
        .file("src/nvm_st.c")
        .compile("libnvm_st.a");
}

