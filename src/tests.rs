/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::ThinBoxedSlice;

    #[test]
    fn option_size() {
        assert_eq!(
            size_of::<Option<ThinBoxedSlice<i32>>>(),
            size_of::<ThinBoxedSlice<i32>>()
        );
    }
}
