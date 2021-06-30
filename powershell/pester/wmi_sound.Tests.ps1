Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_sound table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_sound = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_sound' }
				$wmi_sound | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_sound = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_sound' }
				$schema = Get-Content $wmi_sound.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_sound = Get-Siq 'wmi_sound' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_sound' }).FullName -Raw

				$json = $wmi_sound | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
