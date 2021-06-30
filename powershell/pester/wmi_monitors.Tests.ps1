Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_monitors table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_monitors = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_monitors' }
				$wmi_monitors | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_monitors = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_monitors' }
				$schema = Get-Content $wmi_monitors.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_monitors = Get-Siq 'wmi_monitors' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_monitors' }).FullName -Raw

				$json = $wmi_monitors | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
