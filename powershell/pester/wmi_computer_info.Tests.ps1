Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_computer_info table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_computer_info = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_computer_info' }
				$wmi_computer_info | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_computer_info = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_computer_info' }
				$schema = Get-Content $wmi_computer_info.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_computer_info = Get-Siq 'wmi_computer_info' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_computer_info' }).FullName -Raw

				$json = $wmi_computer_info | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
