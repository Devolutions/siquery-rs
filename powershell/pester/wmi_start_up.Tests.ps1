Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_start_up table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_start_up = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_start_up' }
				$wmi_start_up | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_start_up = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_start_up' }
				$schema = Get-Content $wmi_start_up.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_start_up = Get-Siq 'wmi_start_up' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_start_up' }).FullName -Raw

				$json = $wmi_start_up | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
