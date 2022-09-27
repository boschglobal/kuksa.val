import setuptools
from setuptools.command import build_py
from setuptools.command import sdist


class BuildPackageProtos:
    def run(self):
        from grpc_tools import command
        command.build_package_protos('.')
        super().run()


class BuildPyCommand(BuildPackageProtos, build_py.build_py):
    ...


class SDistCommand(BuildPackageProtos, sdist.sdist):
    ...


setuptools.setup(
    cmdclass={
        'build_py': BuildPyCommand,  # Used for editable installs but also for building wheels
        'sdist': SDistCommand,
    }
)
