/*
 * Copyright (c) 2025-2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#![cfg(feature = "std")]

use alloc::format;
use blacknet_crypto::integer::Integer;
use blacknet_crypto::random::{FAST_RNG, UniformGenerator};
use core::error::Error;
use std::fs::{File, remove_file, rename};
use std::io::{BufWriter, Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};

pub fn create_temp_file(dir: &Path, prefix: &str) -> Result<(PathBuf, File), IoError> {
    FAST_RNG.with_borrow_mut(|rng| {
        loop {
            let name = format!("{}-{}", prefix, rng.generate().cast_unsigned());
            let path = dir.join(name);
            match File::create_new(&path) {
                Ok(file) => return Ok((path, file)),
                Err(err) => {
                    if err.kind() == ErrorKind::AlreadyExists {
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    })
}

// Atomically replace file
pub fn replace<E: Error + From<IoError>>(
    dir: &Path,
    name: &str,
    writer: impl FnOnce(&mut BufWriter<File>) -> Result<(), E>,
) -> Result<(), E> {
    let (path, file) = create_temp_file(dir, name)?;
    let result = replace_impl(dir, name, &path, file, writer);
    if result.is_err() {
        let _ = remove_file(path);
        // Ignore failed cleanup
    }
    result
}

fn replace_impl<E: Error + From<IoError>>(
    dir: &Path,
    name: &str,
    path: &Path,
    file: File,
    writer: impl FnOnce(&mut BufWriter<File>) -> Result<(), E>,
) -> Result<(), E> {
    let mut buffered = BufWriter::new(file);
    writer(&mut buffered)?;
    let file = match buffered.into_inner() {
        Ok(file) => Ok(file),
        Err(err) => Err(err.into_error()),
    }?;
    file.sync_data()?;
    drop(file);
    Ok(rename(path, dir.join(name))?)
}
