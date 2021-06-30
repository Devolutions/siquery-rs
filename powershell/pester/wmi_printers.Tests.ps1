Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_printers table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_printers = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_printers' }
				$wmi_printers | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_printers = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_printers' }
				$schema = Get-Content $wmi_printers.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_printers = Get-Siq 'wmi_printers' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_printers' }).FullName -Raw

				$json = $wmi_printers | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
