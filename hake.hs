{-# LANGUAGE MultiWayIf    #-}
{-# LANGUAGE UnicodeSyntax #-}

import Hake

main ∷ IO ()
main = hake $ do

  "clean | clean the project" ∫
    cargo ["clean"] ?> removeDirIfExists targetPath

  "update | update dependencies" ∫ cargo ["update"]

  kcExecutable ♯
    cargo <| "build" : buildFlags

  "install | install to system" ∫
    cargo <| "install" : buildFlags

  "run | run KC" ◉ [ kcExecutable ] ∰ do
    cargo . (("run" : buildFlags) ++) . ("--" :) =<< getHakeArgs

 where
  appNameKC ∷ String
  appNameKC = "kalmarity-control"

  targetPath ∷ FilePath
  targetPath = "target"

  buildPath ∷ FilePath
  buildPath = targetPath </> "release"

  buildFlags ∷ [String]
  buildFlags = [ "--release" ]

  kcExecutable ∷ FilePath
  kcExecutable =
    {- HLINT ignore "Redundant multi-way if" -}
    if | os ∈ ["win32", "mingw32", "cygwin32"] → buildPath </> appNameKC ++ ".exe"
       | otherwise → buildPath </> appNameKC
