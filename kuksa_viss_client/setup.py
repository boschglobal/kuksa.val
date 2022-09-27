import distutils.command.build
import setuptools


class BuildCommand(distutils.command.build.build):
    def run(self):
        from grpc_tools import command
        command.build_package_protos('.')
        super().run()


setuptools.setup(
    cmdclass={
        'build': BuildCommand,
    }
)
