#
# Be sure to run `pod lib lint HydraMath.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'HydraMath'
  s.version          = '0.2.0'
  s.summary          = 'Swift wrapper for hydra math'
  s.homepage         = 'https://github.com/novasamatech/hydra-math-swift'
  s.author           = { 'Ruslan Rezin' => 'ruslan@novawallet.io' }
  s.source           = { :git => 'https://github.com/novasamatech/hydra-math-swift.git', :tag => s.version.to_s }

  s.ios.deployment_target = '12.0'
  s.swift_version = '5.0'

  s.vendored_frameworks = 'bindings/xcframework/hydra_dx.xcframework'
  s.source_files = 'Sources/**/*.swift'
end