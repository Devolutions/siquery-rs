Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_physical_memory table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_physical_memory = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_physical_memory' }
				$wmi_physical_memory | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_physical_memory = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_physical_memory' }
				$schema = Get-Content $wmi_physical_memory.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_physical_memory = Get-Siq 'wmi_physical_memory' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_physical_memory' }).FullName -Raw

				$json = $wmi_physical_memory | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
