Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_services table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_services = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_services' }
				$wmi_services | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_services = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_services' }
				$schema = Get-Content $wmi_services.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_services = Get-Siq 'wmi_services' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_services' }).FullName -Raw

				$json = $wmi_services | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
