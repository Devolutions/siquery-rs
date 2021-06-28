Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_pointing_device table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_pointing_device = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_pointing_device' }
				$wmi_pointing_device | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_pointing_device = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_pointing_device' }
				$schema = Get-Content $wmi_pointing_device.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_pointing_device = Get-Siq 'wmi_pointing_device' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_pointing_device' }).FullName -Raw

				$json = $wmi_pointing_device | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
