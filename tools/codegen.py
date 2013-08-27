# This file is part of Grust, GObject introspection bindings for Rust
#
# Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
#
# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
# 02110-1301  USA

from giscanner.girparser import GIRParser
from uuid import uuid4

# TODO: substitute by configure
GRUST_VERSION='0.1'

_fmt_crate_prolog="""// This is a generated file. Do not edit.

#[link(name="{crate_name}", vers="{vers}", uuid="{uuid}")];

"""

_fmt_extern_mod="""extern mod {name} (name="{crate_name}", vers="{vers}");
"""

def link_name(name):
    return 'grust-' + name

class RustCodeGenerator(object):
    def __init__(self, in_filename, out_filename, uuid=None):
        self.in_filename = in_filename
        self.out_filename = out_filename
        self.uuid = uuid or uuid4()

    def codegen(self):
        parser = GIRParser()
        parser.parse(self.in_filename)
        ns = parser.get_namespace()
        self.out = open(self.out_filename, 'w')
        self.out.write(_fmt_crate_prolog.format(crate_name=link_name(ns.name),
                                                vers=ns.version,
                                                uuid=self.uuid))
        self._gen_extern_mod("grust", GRUST_VERSION, crate_name="grust")
        for include in ns.includes:
            # TODO: follow includes transitively to import all extern modules?
            self._gen_extern_mod(include.name, include.version)

    def _gen_extern_mod(self, name, version, crate_name=None):
        if crate_name is None:
            crate_name = link_name(name)
        self.out.write(_fmt_extern_mod.format(
                name=name.lower(), vers=version, crate_name=crate_name))

if __name__ == '__main__':
    # TODO: argparse
    from sys import argv
    in_filename = argv[1]
    out_filename = argv[2]
    gen = RustCodeGenerator(in_filename, out_filename)
    gen.codegen()
