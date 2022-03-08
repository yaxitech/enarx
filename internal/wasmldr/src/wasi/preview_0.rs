// SPDX-License-Identifier: Apache-2.0

use super::Ctx;

use wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable;
use wasi_common::Error;
use wasi_common::{snapshots::preview_0::types, ErrorExt};
use wiggle::{GuestPtr, Trap};

impl types::UserErrorConversion for Ctx {
    fn errno_from_error(&mut self, e: Error) -> Result<types::Errno, Trap> {
        self.inner.errno_from_error(e)
    }
}

#[wiggle::async_trait]
impl WasiUnstable for Ctx {
    async fn args_get<'a>(
        &mut self,
        argv: &GuestPtr<'a, GuestPtr<'a, u8>>,
        argv_buf: &GuestPtr<'a, u8>,
    ) -> Result<(), Error> {
        WasiUnstable::args_get(&mut self.inner, argv, argv_buf).await
    }

    async fn args_sizes_get(&mut self) -> Result<(types::Size, types::Size), Error> {
        WasiUnstable::args_sizes_get(&mut self.inner).await
    }

    async fn environ_get<'a>(
        &mut self,
        environ: &GuestPtr<'a, GuestPtr<'a, u8>>,
        environ_buf: &GuestPtr<'a, u8>,
    ) -> Result<(), Error> {
        WasiUnstable::environ_get(&mut self.inner, environ, environ_buf).await
    }

    async fn environ_sizes_get(&mut self) -> Result<(types::Size, types::Size), Error> {
        WasiUnstable::environ_sizes_get(&mut self.inner).await
    }

    async fn clock_res_get(&mut self, id: types::Clockid) -> Result<types::Timestamp, Error> {
        WasiUnstable::clock_res_get(&mut self.inner, id).await
    }

    async fn clock_time_get(
        &mut self,
        id: types::Clockid,
        precision: types::Timestamp,
    ) -> Result<types::Timestamp, Error> {
        WasiUnstable::clock_time_get(&mut self.inner, id, precision).await
    }

    async fn fd_advise(
        &mut self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> Result<(), Error> {
        WasiUnstable::fd_advise(&mut self.inner, fd, offset, len, advice).await
    }

    async fn fd_allocate(
        &mut self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
    ) -> Result<(), Error> {
        WasiUnstable::fd_allocate(&mut self.inner, fd, offset, len).await
    }

    async fn fd_close(&mut self, fd: types::Fd) -> Result<(), Error> {
        WasiUnstable::fd_close(&mut self.inner, fd).await
    }

    async fn fd_datasync(&mut self, fd: types::Fd) -> Result<(), Error> {
        WasiUnstable::fd_datasync(&mut self.inner, fd).await
    }

    async fn fd_fdstat_get(&mut self, fd: types::Fd) -> Result<types::Fdstat, Error> {
        WasiUnstable::fd_fdstat_get(&mut self.inner, fd).await
    }

    async fn fd_fdstat_set_flags(
        &mut self,
        fd: types::Fd,
        flags: types::Fdflags,
    ) -> Result<(), Error> {
        WasiUnstable::fd_fdstat_set_flags(&mut self.inner, fd, flags).await
    }

    async fn fd_fdstat_set_rights(
        &mut self,
        fd: types::Fd,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
    ) -> Result<(), Error> {
        WasiUnstable::fd_fdstat_set_rights(
            &mut self.inner,
            fd,
            fs_rights_base,
            fs_rights_inheriting,
        )
        .await
    }

    async fn fd_filestat_get(&mut self, fd: types::Fd) -> Result<types::Filestat, Error> {
        WasiUnstable::fd_filestat_get(&mut self.inner, fd).await
    }

    async fn fd_filestat_set_size(
        &mut self,
        fd: types::Fd,
        size: types::Filesize,
    ) -> Result<(), Error> {
        WasiUnstable::fd_filestat_set_size(&mut self.inner, fd, size).await
    }

    async fn fd_filestat_set_times(
        &mut self,
        fd: types::Fd,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), Error> {
        WasiUnstable::fd_filestat_set_times(&mut self.inner, fd, atim, mtim, fst_flags).await
    }

    async fn fd_read<'a>(
        &mut self,
        fd: types::Fd,
        iovs: &types::IovecArray<'a>,
    ) -> Result<types::Size, Error> {
        WasiUnstable::fd_read(&mut self.inner, fd, iovs).await
    }

    async fn fd_pread<'a>(
        &mut self,
        fd: types::Fd,
        iovs: &types::IovecArray<'a>,
        offset: types::Filesize,
    ) -> Result<types::Size, Error> {
        WasiUnstable::fd_pread(&mut self.inner, fd, iovs, offset).await
    }

    async fn fd_write<'a>(
        &mut self,
        fd: types::Fd,
        ciovs: &types::CiovecArray<'a>,
    ) -> Result<types::Size, Error> {
        WasiUnstable::fd_write(&mut self.inner, fd, ciovs).await
    }

    async fn fd_pwrite<'a>(
        &mut self,
        fd: types::Fd,
        ciovs: &types::CiovecArray<'a>,
        offset: types::Filesize,
    ) -> Result<types::Size, Error> {
        WasiUnstable::fd_pwrite(&mut self.inner, fd, ciovs, offset).await
    }

    async fn fd_prestat_get(&mut self, fd: types::Fd) -> Result<types::Prestat, Error> {
        WasiUnstable::fd_prestat_get(&mut self.inner, fd).await
    }

    async fn fd_prestat_dir_name<'a>(
        &mut self,
        fd: types::Fd,
        path: &GuestPtr<'a, u8>,
        path_max_len: types::Size,
    ) -> Result<(), Error> {
        WasiUnstable::fd_prestat_dir_name(&mut self.inner, fd, path, path_max_len).await
    }

    async fn fd_renumber(&mut self, from: types::Fd, to: types::Fd) -> Result<(), Error> {
        WasiUnstable::fd_renumber(&mut self.inner, from, to).await
    }

    async fn fd_seek(
        &mut self,
        fd: types::Fd,
        offset: types::Filedelta,
        whence: types::Whence,
    ) -> Result<types::Filesize, Error> {
        WasiUnstable::fd_seek(&mut self.inner, fd, offset, whence).await
    }

    async fn fd_sync(&mut self, fd: types::Fd) -> Result<(), Error> {
        WasiUnstable::fd_sync(&mut self.inner, fd).await
    }

    async fn fd_tell(&mut self, fd: types::Fd) -> Result<types::Filesize, Error> {
        WasiUnstable::fd_tell(&mut self.inner, fd).await
    }

    async fn fd_readdir<'a>(
        &mut self,
        fd: types::Fd,
        buf: &GuestPtr<'a, u8>,
        buf_len: types::Size,
        cookie: types::Dircookie,
    ) -> Result<types::Size, Error> {
        WasiUnstable::fd_readdir(&mut self.inner, fd, buf, buf_len, cookie).await
    }

    async fn path_create_directory<'a>(
        &mut self,
        dirfd: types::Fd,
        path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_create_directory(&mut self.inner, dirfd, path).await
    }

    async fn path_filestat_get<'a>(
        &mut self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'a, str>,
    ) -> Result<types::Filestat, Error> {
        WasiUnstable::path_filestat_get(&mut self.inner, dirfd, flags, path).await
    }

    async fn path_filestat_set_times<'a>(
        &mut self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'a, str>,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), Error> {
        WasiUnstable::path_filestat_set_times(
            &mut self.inner,
            dirfd,
            flags,
            path,
            atim,
            mtim,
            fst_flags,
        )
        .await
    }

    async fn path_link<'a>(
        &mut self,
        src_fd: types::Fd,
        src_flags: types::Lookupflags,
        src_path: &GuestPtr<'a, str>,
        target_fd: types::Fd,
        target_path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_link(
            &mut self.inner,
            src_fd,
            src_flags,
            src_path,
            target_fd,
            target_path,
        )
        .await
    }

    async fn path_open<'a>(
        &mut self,
        dirfd: types::Fd,
        dirflags: types::Lookupflags,
        path: &GuestPtr<'a, str>,
        oflags: types::Oflags,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
        fdflags: types::Fdflags,
    ) -> Result<types::Fd, Error> {
        WasiUnstable::path_open(
            &mut self.inner,
            dirfd,
            dirflags,
            path,
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fdflags,
        )
        .await
    }

    async fn path_readlink<'a>(
        &mut self,
        dirfd: types::Fd,
        path: &GuestPtr<'a, str>,
        buf: &GuestPtr<'a, u8>,
        buf_len: types::Size,
    ) -> Result<types::Size, Error> {
        WasiUnstable::path_readlink(&mut self.inner, dirfd, path, buf, buf_len).await
    }

    async fn path_remove_directory<'a>(
        &mut self,
        dirfd: types::Fd,
        path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_remove_directory(&mut self.inner, dirfd, path).await
    }

    async fn path_rename<'a>(
        &mut self,
        src_fd: types::Fd,
        src_path: &GuestPtr<'a, str>,
        dest_fd: types::Fd,
        dest_path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_rename(&mut self.inner, src_fd, src_path, dest_fd, dest_path).await
    }

    async fn path_symlink<'a>(
        &mut self,
        src_path: &GuestPtr<'a, str>,
        dirfd: types::Fd,
        dest_path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_symlink(&mut self.inner, src_path, dirfd, dest_path).await
    }

    async fn path_unlink_file<'a>(
        &mut self,
        dirfd: types::Fd,
        path: &GuestPtr<'a, str>,
    ) -> Result<(), Error> {
        WasiUnstable::path_unlink_file(&mut self.inner, dirfd, path).await
    }

    async fn poll_oneoff<'a>(
        &mut self,
        subs: &GuestPtr<'a, types::Subscription>,
        events: &GuestPtr<'a, types::Event>,
        nsubscriptions: types::Size,
    ) -> Result<types::Size, Error> {
        WasiUnstable::poll_oneoff(&mut self.inner, subs, events, nsubscriptions).await
    }

    async fn proc_exit(&mut self, status: types::Exitcode) -> wiggle::Trap {
        WasiUnstable::proc_exit(&mut self.inner, status).await
    }

    async fn proc_raise(&mut self, _sig: types::Signal) -> Result<(), Error> {
        Err(Error::trap("proc_raise unsupported"))
    }

    async fn sched_yield(&mut self) -> Result<(), Error> {
        WasiUnstable::sched_yield(&mut self.inner).await
    }

    async fn random_get<'a>(
        &mut self,
        buf: &GuestPtr<'a, u8>,
        buf_len: types::Size,
    ) -> Result<(), Error> {
        WasiUnstable::random_get(&mut self.inner, buf, buf_len).await
    }

    async fn sock_recv<'a>(
        &mut self,
        _fd: types::Fd,
        _ri_data: &types::IovecArray<'a>,
        _ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags), Error> {
        Err(Error::trap("sock_recv unsupported"))
    }

    async fn sock_send<'a>(
        &mut self,
        _fd: types::Fd,
        _si_data: &types::CiovecArray<'a>,
        _si_flags: types::Siflags,
    ) -> Result<types::Size, Error> {
        Err(Error::trap("sock_send unsupported"))
    }

    async fn sock_shutdown(&mut self, _fd: types::Fd, _how: types::Sdflags) -> Result<(), Error> {
        Err(Error::trap("sock_shutdown unsupported"))
    }
}