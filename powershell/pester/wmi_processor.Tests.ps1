Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_processor table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_processor = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_processor' }
				$wmi_processor | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_processor = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_processor' }
				$schema = Get-Content $wmi_processor.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_processor = Get-Siq 'wmi_processor' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_processor' }).FullName -Raw

				$json = $wmi_processor | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
