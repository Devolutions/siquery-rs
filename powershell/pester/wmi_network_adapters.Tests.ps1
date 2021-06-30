Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_network_adapters table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_network_adapters = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_network_adapters' }
				$wmi_network_adapters | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_network_adapters = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_network_adapters' }
				$schema = Get-Content $wmi_network_adapters.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_network_adapters = Get-Siq 'wmi_network_adapters' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_network_adapters' }).FullName -Raw

				$json = $wmi_network_adapters | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
