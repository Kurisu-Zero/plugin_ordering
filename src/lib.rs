#![feature(fmt_internals)]
#![feature(core_panic)]
#![feature(libstd_sys_internals)]
mod mocks;
mod release;
#[cfg(test)]
mod release_copy;

#[cfg(test)]
mod tests {

    mod tests_using_mocks;
    mod tests_without_mocks;
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}
