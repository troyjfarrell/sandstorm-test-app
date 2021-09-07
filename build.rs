extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .import_path("/opt/sandstorm/latest/usr/include")
        .src_prefix("/opt/sandstorm/latest/usr/include")
        .file("test-app.capnp")
        // The code generator seems to think that imports belong to the same crate.  I have not
        // found a way to refer to a different crate, so I'm importing these temporarily.
        .file("/opt/sandstorm/latest/usr/include/capnp/persistent.capnp")
        .file("/opt/sandstorm/latest/usr/include/capnp/stream.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/activity.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/api-session.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/grain.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/identity.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/ip.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/package.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/powerbox.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/supervisor.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/util.capnp")
        .file("/opt/sandstorm/latest/usr/include/sandstorm/web-session.capnp")
        .run()
        .expect("compiling");
}
