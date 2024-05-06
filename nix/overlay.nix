final: prev: {
  demostf-parser = final.callPackage ./parser.nix {};
  demostf-parser-codegen = final.callPackage ./codegen.nix {};
  demostf-parser-codegen-events = final.runCommand "gameevent_gen.rs" {} ''
    ${final.demostf-parser-codegen}/bin/codegen ${../test_data/short-2024.dem} events > $out
    ${final.rustfmt}/bin/rustfmt $out
  '';
  demostf-parser-codegen-props = final.runCommand "sendprop_gen.rs" {} ''
    ${final.demostf-parser-codegen}/bin/codegen ${../test_data/short-2024.dem} props > $out
    ${final.rustfmt}/bin/rustfmt $out
  '';
  demostf-parser-schema = final.callPackage ./schema.nix {};

}
