{
    description = "Abnormality detection in videos using PD-ED Model";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }:
    let
        pkgs = import nixpkgs {
            system = "x86_64-linux";
        };
    in
    {
        devShells.${pkgs.system}.default =
            pkgs.mkShell {
                buildInputs = with pkgs; [
                    gcc
                    cudatoolkit
                    cudaPackages.cuda_cudart
                    python312
                    python312Packages.tensorflowWithCuda
                    python312Packages.distutils
                    python312Packages.opencv4
                    (python312Packages.keras.override {
                     tensorflow = python312Packages.tensorflowWithCuda;
                     })
                    python312Packages.flask
                ];
                shellHook = ''
                    echo "welcome the the pded project environment"
                    export NIX_DEV_SHELL='pded'
                    export CUDA_PATH='${pkgs.cudatoolkit}'
                    export XLA_FLAGS='--xla_gpu_cuda_data_dir=${pkgs.cudatoolkit}'
                    export CUDA_DIR='${pkgs.cudatoolkit}'
                    export LD_LIBRARY_PATH='$LD_LIBRARY_APTH:${pkgs.cudatoolkit}/lib'
                '';
            };
    };
}
